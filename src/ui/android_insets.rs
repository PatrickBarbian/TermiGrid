/// Logical safe-area insets (in `egui` points) so UI elements stay clear of Android
/// status bars, navigation gesture bars, soft keyboards, and camera display cutouts.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SafeAreaInsets {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

#[cfg(target_os = "android")]
mod imp {
    use std::ffi::c_void;
    use std::mem::ManuallyDrop;
    use std::sync::OnceLock;

    use jni::objects::{JObject, JValue};
    use jni::{jni_sig, jni_str, Env, JavaVM};

    use super::SafeAreaInsets;

    static ANDROID_VM_PTR: OnceLock<usize> = OnceLock::new();
    static ANDROID_ACTIVITY_PTR: OnceLock<usize> = OnceLock::new();

    pub fn init_android_refs(vm_ptr: *mut c_void, activity_ptr: *mut c_void) {
        if !vm_ptr.is_null() {
            let _ = ANDROID_VM_PTR.set(vm_ptr as usize);
        }
        if !activity_ptr.is_null() {
            let _ = ANDROID_ACTIVITY_PTR.set(activity_ptr as usize);
        }
    }

    pub fn android_jvm() -> Option<JavaVM> {
        let &vm_addr = ANDROID_VM_PTR.get()?;
        if vm_addr == 0 {
            return None;
        }
        Some(unsafe { JavaVM::from_raw(vm_addr as *mut jni::sys::JavaVM) })
    }

    pub fn current_safe_insets(pixels_per_point: f32) -> SafeAreaInsets {
        let Some(&vm_addr) = ANDROID_VM_PTR.get() else {
            return SafeAreaInsets::default();
        };
        let Some(&act_addr) = ANDROID_ACTIVITY_PTR.get() else {
            return SafeAreaInsets::default();
        };
        if vm_addr == 0 || act_addr == 0 {
            return SafeAreaInsets::default();
        }

        let jvm = unsafe { JavaVM::from_raw(vm_addr as *mut jni::sys::JavaVM) };
        let px = jvm
            .attach_current_thread(|env| -> jni::errors::Result<[i32; 4]> {
                Ok(query_inset_pixels(env, act_addr as jni::sys::jobject))
            })
            .unwrap_or([0, 0, 0, 0]);

        let ppp = pixels_per_point.max(1.0);
        SafeAreaInsets {
            top: (px[0].max(0) as f32) / ppp,
            bottom: (px[1].max(0) as f32) / ppp,
            left: (px[2].max(0) as f32) / ppp,
            right: (px[3].max(0) as f32) / ppp,
        }
    }

    fn clear_if_exception(env: &Env<'_>) {
        if env.exception_check() {
            env.exception_clear();
        }
    }

    /// Returns physical pixel insets `[top, bottom, left, right]`.
    fn query_inset_pixels(env: &mut Env<'_>, raw_activity: jni::sys::jobject) -> [i32; 4] {
        // NativeActivity::clazz is a global ref; ManuallyDrop prevents wrapping it as an owned local.
        let activity = ManuallyDrop::new(unsafe { JObject::from_raw(env, raw_activity) });

        let sdk_int = env
            .get_static_field(
                jni_str!("android/os/Build$VERSION"),
                jni_str!("SDK_INT"),
                jni_sig!("I"),
            )
            .and_then(|v| v.i())
            .unwrap_or(24);
        clear_if_exception(env);

        let Ok(window) = env
            .call_method(
                &*activity,
                jni_str!("getWindow"),
                jni_sig!("()Landroid/view/Window;"),
                &[],
            )
            .and_then(|v| v.l())
        else {
            clear_if_exception(env);
            return fallback_system_bar_dimens(env, &activity);
        };
        if window.is_null() {
            return fallback_system_bar_dimens(env, &activity);
        }

        let Ok(decor) = env
            .call_method(
                &window,
                jni_str!("getDecorView"),
                jni_sig!("()Landroid/view/View;"),
                &[],
            )
            .and_then(|v| v.l())
        else {
            clear_if_exception(env);
            return fallback_system_bar_dimens(env, &activity);
        };
        if decor.is_null() {
            return fallback_system_bar_dimens(env, &activity);
        }

        let Ok(insets) = env
            .call_method(
                &decor,
                jni_str!("getRootWindowInsets"),
                jni_sig!("()Landroid/view/WindowInsets;"),
                &[],
            )
            .and_then(|v| v.l())
        else {
            clear_if_exception(env);
            return fallback_system_bar_dimens(env, &activity);
        };

        if insets.is_null() {
            return fallback_system_bar_dimens(env, &activity);
        }

        if sdk_int >= 30 {
            if let Some(px) = query_api30_insets(env, &insets) {
                return px;
            }
            clear_if_exception(env);
        }

        query_legacy_insets(env, &insets, sdk_int)
    }

    fn query_api30_insets(env: &mut Env<'_>, insets: &JObject<'_>) -> Option<[i32; 4]> {
        let sys_bars = env
            .call_static_method(
                jni_str!("android/view/WindowInsets$Type"),
                jni_str!("systemBars"),
                jni_sig!("()I"),
                &[],
            )
            .and_then(|v| v.i())
            .ok()?;

        let cutout = env
            .call_static_method(
                jni_str!("android/view/WindowInsets$Type"),
                jni_str!("displayCutout"),
                jni_sig!("()I"),
                &[],
            )
            .and_then(|v| v.i())
            .ok()?;

        let ime = env
            .call_static_method(
                jni_str!("android/view/WindowInsets$Type"),
                jni_str!("ime"),
                jni_sig!("()I"),
                &[],
            )
            .and_then(|v| v.i())
            .unwrap_or(0);

        let bar_obj = env
            .call_method(
                insets,
                jni_str!("getInsetsIgnoringVisibility"),
                jni_sig!("(I)Landroid/graphics/Insets;"),
                &[JValue::Int(sys_bars | cutout)],
            )
            .and_then(|v| v.l())
            .ok()?;

        if bar_obj.is_null() {
            return None;
        }

        let top = env
            .get_field(&bar_obj, jni_str!("top"), jni_sig!("I"))
            .and_then(|v| v.i())
            .ok()?;
        let mut bottom = env
            .get_field(&bar_obj, jni_str!("bottom"), jni_sig!("I"))
            .and_then(|v| v.i())
            .ok()?;
        let left = env
            .get_field(&bar_obj, jni_str!("left"), jni_sig!("I"))
            .and_then(|v| v.i())
            .ok()?;
        let right = env
            .get_field(&bar_obj, jni_str!("right"), jni_sig!("I"))
            .and_then(|v| v.i())
            .ok()?;

        if ime != 0
            && let Ok(ime_obj) = env
                .call_method(
                    insets,
                    jni_str!("getInsets"),
                    jni_sig!("(I)Landroid/graphics/Insets;"),
                    &[JValue::Int(ime)],
                )
                .and_then(|v| v.l())
            && !ime_obj.is_null()
        {
            let ime_bottom = env
                .get_field(&ime_obj, jni_str!("bottom"), jni_sig!("I"))
                .and_then(|v| v.i())
                .unwrap_or(0);
            bottom = bottom.max(ime_bottom);
        }

        Some([top, bottom, left, right])
    }

    fn query_legacy_insets(env: &mut Env<'_>, insets: &JObject<'_>, sdk_int: i32) -> [i32; 4] {
        let mut top = env
            .call_method(
                insets,
                jni_str!("getSystemWindowInsetTop"),
                jni_sig!("()I"),
                &[],
            )
            .and_then(|v| v.i())
            .unwrap_or(0);
        clear_if_exception(env);

        let mut bottom = env
            .call_method(
                insets,
                jni_str!("getSystemWindowInsetBottom"),
                jni_sig!("()I"),
                &[],
            )
            .and_then(|v| v.i())
            .unwrap_or(0);
        clear_if_exception(env);

        let mut left = env
            .call_method(
                insets,
                jni_str!("getSystemWindowInsetLeft"),
                jni_sig!("()I"),
                &[],
            )
            .and_then(|v| v.i())
            .unwrap_or(0);
        clear_if_exception(env);

        let mut right = env
            .call_method(
                insets,
                jni_str!("getSystemWindowInsetRight"),
                jni_sig!("()I"),
                &[],
            )
            .and_then(|v| v.i())
            .unwrap_or(0);
        clear_if_exception(env);

        if sdk_int >= 28
            && let Ok(cutout) = env
                .call_method(
                    insets,
                    jni_str!("getDisplayCutout"),
                    jni_sig!("()Landroid/view/DisplayCutout;"),
                    &[],
                )
                .and_then(|v| v.l())
            && !cutout.is_null()
        {
            top = top.max(
                env.call_method(&cutout, jni_str!("getSafeInsetTop"), jni_sig!("()I"), &[])
                    .and_then(|v| v.i())
                    .unwrap_or(0),
            );
            bottom = bottom.max(
                env.call_method(
                    &cutout,
                    jni_str!("getSafeInsetBottom"),
                    jni_sig!("()I"),
                    &[],
                )
                .and_then(|v| v.i())
                .unwrap_or(0),
            );
            left = left.max(
                env.call_method(&cutout, jni_str!("getSafeInsetLeft"), jni_sig!("()I"), &[])
                    .and_then(|v| v.i())
                    .unwrap_or(0),
            );
            right = right.max(
                env.call_method(&cutout, jni_str!("getSafeInsetRight"), jni_sig!("()I"), &[])
                    .and_then(|v| v.i())
                    .unwrap_or(0),
            );
        }
        clear_if_exception(env);

        [top, bottom, left, right]
    }

    fn fallback_system_bar_dimens(env: &mut Env<'_>, activity: &JObject<'_>) -> [i32; 4] {
        let Ok(res) = env
            .call_method(
                activity,
                jni_str!("getResources"),
                jni_sig!("()Landroid/content/res/Resources;"),
                &[],
            )
            .and_then(|v| v.l())
        else {
            clear_if_exception(env);
            return [0, 0, 0, 0];
        };
        if res.is_null() {
            return [0, 0, 0, 0];
        }

        let top = read_android_dimen_px(env, &res, "status_bar_height");
        let bottom = read_android_dimen_px(env, &res, "navigation_bar_height");
        [top, bottom, 0, 0]
    }

    fn read_android_dimen_px(env: &mut Env<'_>, res: &JObject<'_>, name: &str) -> i32 {
        let Ok(jname) = env.new_string(name) else {
            clear_if_exception(env);
            return 0;
        };
        let Ok(jtype) = env.new_string("dimen") else {
            clear_if_exception(env);
            return 0;
        };
        let Ok(jpkg) = env.new_string("android") else {
            clear_if_exception(env);
            return 0;
        };

        let Ok(id) = env
            .call_method(
                res,
                jni_str!("getIdentifier"),
                jni_sig!("(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)I"),
                &[
                    JValue::Object(&jname),
                    JValue::Object(&jtype),
                    JValue::Object(&jpkg),
                ],
            )
            .and_then(|v| v.i())
        else {
            clear_if_exception(env);
            return 0;
        };

        if id <= 0 {
            return 0;
        }

        let px = env
            .call_method(
                res,
                jni_str!("getDimensionPixelSize"),
                jni_sig!("(I)I"),
                &[JValue::Int(id)],
            )
            .and_then(|v| v.i())
            .unwrap_or(0);
        clear_if_exception(env);
        px.max(0)
    }

    pub fn open_android_url(url: &str) -> bool {
        let Some(&vm_addr) = ANDROID_VM_PTR.get() else {
            return false;
        };
        let Some(&act_addr) = ANDROID_ACTIVITY_PTR.get() else {
            return false;
        };
        if vm_addr == 0 || act_addr == 0 {
            return false;
        }

        let jvm = unsafe { JavaVM::from_raw(vm_addr as *mut jni::sys::JavaVM) };
        let res = jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
            let activity = ManuallyDrop::new(unsafe { JObject::from_raw(env, act_addr as jni::sys::jobject) });
            let jurl = env.new_string(url)?;
            let uri = env.call_static_method(
                jni_str!("android/net/Uri"),
                jni_str!("parse"),
                jni_sig!("(Ljava/lang/String;)Landroid/net/Uri;"),
                &[JValue::Object(&jurl)],
            )?.l()?;

            let action_view = env.get_static_field(
                jni_str!("android/content/Intent"),
                jni_str!("ACTION_VIEW"),
                jni_sig!("Ljava/lang/String;"),
            )?.l()?;

            let intent = env.new_object(
                jni_str!("android/content/Intent"),
                jni_sig!("(Ljava/lang/String;Landroid/net/Uri;)V"),
                &[JValue::Object(&action_view), JValue::Object(&uri)],
            )?;

            let _ = env.call_method(
                &intent,
                jni_str!("addFlags"),
                jni_sig!("(I)Landroid/content/Intent;"),
                &[JValue::Int(0x1000_0000)],
            );

            env.call_method(
                &*activity,
                jni_str!("startActivity"),
                jni_sig!("(Landroid/content/Intent;)V"),
                &[JValue::Object(&intent)],
            )?;
            Ok(())
        });
        res.is_ok()
    }
}

#[cfg(not(target_os = "android"))]
mod imp {
    use super::SafeAreaInsets;

    pub fn current_safe_insets(_pixels_per_point: f32) -> SafeAreaInsets {
        SafeAreaInsets::default()
    }

    pub fn open_android_url(_url: &str) -> bool {
        false
    }
}

#[cfg(target_os = "android")]
pub use imp::{android_jvm, init_android_refs};
pub use imp::{current_safe_insets, open_android_url};

