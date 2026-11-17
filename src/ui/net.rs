//! Lightweight cross-platform HTTPS GET helper (`curl` on Desktop, `HttpURLConnection` via JNI on Android)
//! with zero extra TLS crate overhead.

/// Fetches raw bytes over HTTP(S), following redirects up to 5 hops.
pub fn http_get_bytes(url: &str) -> Result<Vec<u8>, String> {
    #[cfg(not(target_os = "android"))]
    {
        let mut cmd = std::process::Command::new("curl");
        cmd.args([
            "-fsSL",
            "--max-redirs",
            "5",
            "-m",
            "8",
            "-H",
            "User-Agent: TermiGrid/1.0",
            url,
        ]);

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let output = cmd.output().map_err(|e| e.to_string())?;
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(err.trim().to_string());
        }
        Ok(output.stdout)
    }

    #[cfg(target_os = "android")]
    {
        android_http_get_bytes(url)
    }
}

/// Fetches UTF-8 text over HTTP(S).
pub fn http_get_text(url: &str) -> Result<String, String> {
    let bytes = http_get_bytes(url)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

#[cfg(target_os = "android")]
fn android_http_get_bytes(start_url: &str) -> Result<Vec<u8>, String> {
    use jni::objects::JValue;
    use jni::{jni_sig, jni_str};

    let jvm = crate::ui::android_insets::android_jvm()
        .ok_or_else(|| "Android JVM not initialized".to_string())?;

    let mut current_url = start_url.to_string();
    for _ in 0..5 {
        let step_result = jvm
            .attach_current_thread(|env| -> jni::errors::Result<Result<Vec<u8>, String>> {
                let jurl_str = env.new_string(&current_url)?;
                let url_obj = env.new_object(
                    jni_str!("java/net/URL"),
                    jni_sig!("(Ljava/lang/String;)V"),
                    &[JValue::Object(&jurl_str)],
                )?;
                let conn = env
                    .call_method(
                        &url_obj,
                        jni_str!("openConnection"),
                        jni_sig!("()Ljava/net/URLConnection;"),
                        &[],
                    )?
                    .l()?;

                let _ = env.call_method(
                    &conn,
                    jni_str!("setConnectTimeout"),
                    jni_sig!("(I)V"),
                    &[JValue::Int(8000)],
                );
                let _ = env.call_method(
                    &conn,
                    jni_str!("setReadTimeout"),
                    jni_sig!("(I)V"),
                    &[JValue::Int(8000)],
                );
                let _ = env.call_method(
                    &conn,
                    jni_str!("setInstanceFollowRedirects"),
                    jni_sig!("(Z)V"),
                    &[JValue::Bool(true)],
                );

                let ua_key = env.new_string("User-Agent")?;
                let ua_val = env.new_string("TermiGrid/1.0")?;
                let _ = env.call_method(
                    &conn,
                    jni_str!("setRequestProperty"),
                    jni_sig!("(Ljava/lang/String;Ljava/lang/String;)V"),
                    &[JValue::Object(&ua_key), JValue::Object(&ua_val)],
                );

                let status = env
                    .call_method(&conn, jni_str!("getResponseCode"), jni_sig!("()I"), &[])?
                    .i()?;

                if (300..400).contains(&status) {
                    let loc_key = env.new_string("Location")?;
                    let loc_obj = env
                        .call_method(
                            &conn,
                            jni_str!("getHeaderField"),
                            jni_sig!("(Ljava/lang/String;)Ljava/lang/String;"),
                            &[JValue::Object(&loc_key)],
                        )?
                        .l()?;
                    let _ = env.call_method(&conn, jni_str!("disconnect"), jni_sig!("()V"), &[]);
                    if !loc_obj.is_null() {
                        let next = unsafe { jni::objects::JString::from_raw(env, loc_obj.into_raw()) }
                            .try_to_string(env)
                            .unwrap_or_default();
                        return Ok(Err(format!("__REDIRECT__:{next}")));
                    }
                }

                if !(200..300).contains(&status) {
                    let _ = env.call_method(&conn, jni_str!("disconnect"), jni_sig!("()V"), &[]);
                    return Ok(Err(format!("HTTP {status}")));
                }

                let stream = env
                    .call_method(
                        &conn,
                        jni_str!("getInputStream"),
                        jni_sig!("()Ljava/io/InputStream;"),
                        &[],
                    )?
                    .l()?;

                let buf_arr = env.new_byte_array(8192)?;
                let mut out = Vec::with_capacity(16384);
                loop {
                    let read_len = env
                        .call_method(
                            &stream,
                            jni_str!("read"),
                            jni_sig!("([B)I"),
                            &[JValue::Object(&buf_arr)],
                        )?
                        .i()?;
                    if read_len <= 0 {
                        break;
                    }
                    let mut tmp = vec![0i8; read_len as usize];
                    buf_arr.get_region(env, 0, &mut tmp)?;
                    out.extend(tmp.into_iter().map(|b| b as u8));
                    if out.len() > 4 * 1024 * 1024 {
                        break;
                    }
                }

                let _ = env.call_method(&stream, jni_str!("close"), jni_sig!("()V"), &[]);
                let _ = env.call_method(&conn, jni_str!("disconnect"), jni_sig!("()V"), &[]);
                Ok(Ok(out))
            })
            .map_err(|e| {
                format!("JNI HTTP error: {e}")
            })?;

        match step_result {
            Ok(bytes) => return Ok(bytes),
            Err(msg) if msg.starts_with("__REDIRECT__:") => {
                let next = msg.trim_start_matches("__REDIRECT__:").to_string();
                if next.is_empty() {
                    return Err("Empty redirect".to_string());
                }
                if next.starts_with('/')
                    && let Some(scheme_end) = current_url.find("://")
                    && let Some(path_rel) = current_url[scheme_end + 3..].find('/')
                {
                    let origin = &current_url[..scheme_end + 3 + path_rel];
                    current_url = format!("{origin}{next}");
                } else {
                    current_url = next;
                }
            }
            Err(e) => return Err(e),
        }
    }

    Err("Too many redirects".to_string())
}
