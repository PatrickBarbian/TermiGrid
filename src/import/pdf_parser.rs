use std::collections::{BTreeMap, HashMap};

use lopdf::content::Content;
use lopdf::{Dictionary, Document, Object, ObjectId};

use crate::import::rules::CompiledImportRules;

/// A positioned text fragment extracted from a PDF page in top-down, left-to-right coordinates.
#[derive(Debug, Clone)]
pub struct PdfTextSpan {
    pub page: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub font_size: f32,
    pub text: String,
}

/// Raw 6-column EPLAN BOM row extracted from the PDF table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawBomRow {
    pub page: u32,
    pub bmk: String,
    pub quantity: usize,
    pub description: String,
    pub type_number: String,
    pub supplier: String,
    pub article_number: String,
}

#[derive(Debug, Clone, Copy)]
struct Matrix([f32; 6]);

impl Matrix {
    fn identity() -> Self {
        Self([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    }

    /// Standard PDF row-vector matrix concatenation: `[a b c d e f]`.
    fn mul(m1: Self, m2: Self) -> Self {
        let [a1, b1, c1, d1, e1, f1] = m1.0;
        let [a2, b2, c2, d2, e2, f2] = m2.0;
        Self([
            a1 * a2 + b1 * c2,
            a1 * b2 + b1 * d2,
            c1 * a2 + d1 * c2,
            c1 * b2 + d1 * d2,
            e1 * a2 + f1 * c2 + e2,
            e1 * b2 + f1 * d2 + f2,
        ])
    }

    fn transform_point(self, x: f32, y: f32) -> (f32, f32) {
        let [a, b, c, d, e, f] = self.0;
        (a * x + c * y + e, b * x + d * y + f)
    }

    fn scale_y(self) -> f32 {
        let [_, _, c, d, _, _] = self.0;
        (c * c + d * d).sqrt().max(0.01)
    }
}

#[derive(Debug, Clone)]
struct GraphicsState {
    ctm: Matrix,
    font_name: Vec<u8>,
    font_size: f32,
    char_space: f32,
    word_space: f32,
    horiz_scale: f32,
    text_leading: f32,
    text_rise: f32,
}

impl Default for GraphicsState {
    fn default() -> Self {
        Self {
            ctm: Matrix::identity(),
            font_name: Vec::new(),
            font_size: 10.0,
            char_space: 0.0,
            word_space: 0.0,
            horiz_scale: 100.0,
            text_leading: 0.0,
            text_rise: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PageBox {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    rotate: i64,
}

impl PageBox {
    fn to_top_down(self, px: f32, py: f32) -> (f32, f32) {
        let w = (self.x1 - self.x0).abs().max(1.0);
        let h = (self.y1 - self.y0).abs().max(1.0);
        let lx = px - self.x0;
        let ly = py - self.y0;
        match self.rotate.rem_euclid(360) {
            90 => (ly, lx),
            180 => (w - lx, ly),
            270 => (h - ly, w - lx),
            _ => (lx, h - ly),
        }
    }
}

fn obj_as_f32(obj: &Object) -> Option<f32> {
    match obj {
        Object::Real(v) => Some(*v),
        Object::Integer(v) => Some(*v as f32),
        _ => None,
    }
}

fn resolve_dict<'a>(doc: &'a Document, obj: &'a Object) -> Option<&'a Dictionary> {
    match obj {
        Object::Dictionary(d) => Some(d),
        Object::Stream(s) => Some(&s.dict),
        Object::Reference(id) => match doc.get_object(*id).ok()? {
            Object::Dictionary(d) => Some(d),
            Object::Stream(s) => Some(&s.dict),
            _ => None,
        },
        _ => None,
    }
}

fn collect_fonts_from_resources<'a>(
    doc: &'a Document,
    resources: &'a Dictionary,
    out: &mut BTreeMap<Vec<u8>, &'a Dictionary>,
) {
    let Ok(font_obj) = resources.get(b"Font") else {
        return;
    };
    let Some(font_dict) = resolve_dict(doc, font_obj) else {
        return;
    };
    for (name, val) in font_dict.iter() {
        if let Some(d) = resolve_dict(doc, val) {
            out.insert(name.clone(), d);
        }
    }
}

fn collect_xobjects_from_resources<'a>(
    doc: &'a Document,
    resources: &'a Dictionary,
    out: &mut HashMap<Vec<u8>, &'a lopdf::Stream>,
) {
    let Ok(xobj) = resources.get(b"XObject") else {
        return;
    };
    let Some(xobj_dict) = resolve_dict(doc, xobj) else {
        return;
    };
    for (name, val) in xobj_dict.iter() {
        let resolved = match val {
            Object::Stream(s) => Some(s),
            Object::Reference(id) => doc.get_object(*id).ok().and_then(|o| o.as_stream().ok()),
            _ => None,
        };
        if let Some(stream) = resolved {
            out.insert(name.clone(), stream);
        }
    }
}

fn get_page_box_and_rotation(doc: &Document, page_id: ObjectId) -> PageBox {
    let mut box_vals: Option<[f32; 4]> = None;
    let mut rotate: i64 = 0;

    let mut cur_id = Some(page_id);
    let mut guard = 0usize;
    while let Some(id) = cur_id {
        if guard > 32 {
            break;
        }
        guard += 1;
        let Ok(dict) = doc.get_dictionary(id) else {
            break;
        };
        if rotate == 0
            && let Ok(r) = dict.get(b"Rotate").and_then(Object::as_i64)
        {
            rotate = r;
        }
        if box_vals.is_none() {
            for key in [b"CropBox".as_slice(), b"MediaBox".as_slice()] {
                if let Ok(arr) = dict.get(key).and_then(Object::as_array)
                    && arr.len() == 4
                    && let (Some(a), Some(b), Some(c), Some(d)) = (
                        obj_as_f32(&arr[0]),
                        obj_as_f32(&arr[1]),
                        obj_as_f32(&arr[2]),
                        obj_as_f32(&arr[3]),
                    )
                {
                    box_vals = Some([a, b, c, d]);
                    break;
                }
            }
        }
        cur_id = dict.get(b"Parent").and_then(Object::as_reference).ok();
    }

    let [x0, y0, x1, y1] = box_vals.unwrap_or([0.0, 0.0, 595.0, 842.0]);
    PageBox {
        x0,
        y0,
        x1,
        y1,
        rotate,
    }
}

/// Decodes PDF string bytes into UTF-8 using BOM, Font `/ToUnicode` / `/Encoding`, or WinAnsi fallback.
fn decode_pdf_bytes(doc: &Document, font_dict: Option<&Dictionary>, bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    // Explicit UTF-16BE BOM
    if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
        let u16s: Vec<u16> = bytes[2..]
            .as_chunks::<2>()
            .0
            .iter()
            .map(|&c| u16::from_be_bytes(c))
            .collect();
        return String::from_utf16_lossy(&u16s);
    }

    if let Some(fdict) = font_dict
        && let Ok(encoding) = fdict.get_font_encoding(doc)
        && let Ok(decoded) = Document::decode_text(&encoding, bytes)
    {
        let cleaned: String = decoded
            .chars()
            .filter(|&c| c != '\u{FFFD}' && (c >= ' ' || c == '\t'))
            .collect();
        if !cleaned.is_empty() {
            return cleaned;
        }
    }

    // Fallback for unmapped Type0/Identity-H 2-byte big-endian BMP text
    if bytes.len() >= 2 && bytes.len().is_multiple_of(2) && bytes.iter().step_by(2).all(|&b| b <= 0x04) {
        let u16s: Vec<u16> = bytes
            .as_chunks::<2>()
            .0
            .iter()
            .map(|&c| u16::from_be_bytes(c))
            .collect();
        let candidate = String::from_utf16_lossy(&u16s);
        if candidate
            .chars()
            .all(|c| !c.is_control() && c != '\u{FFFD}')
            && candidate.chars().any(|c| !c.is_whitespace())
        {
            return candidate;
        }
    }

    // WinAnsi single-byte fallback
    bytes.iter().map(|&b| winansi_to_char(b)).collect()
}

fn winansi_to_char(b: u8) -> char {
    match b {
        0x20..=0x7E => b as char,
        0x80 => '€',
        0x82 => '‚',
        0x84 => '„',
        0x85 => '…',
        0x8A => 'Š',
        0x8C => 'Œ',
        0x8E => 'Ž',
        0x91 => '‘',
        0x92 => '’',
        0x93 => '“',
        0x94 => '”',
        0x95 => '•',
        0x96 => '–',
        0x97 => '—',
        0x9A => 'š',
        0x9C => 'œ',
        0x9E => 'ž',
        0x9F => 'Ÿ',
        0xA0 => ' ',
        0xA1..=0xFF => char::from(b),
        _ => ' ',
    }
}

#[allow(clippy::too_many_arguments)]
fn extract_stream_spans(
    doc: &Document,
    raw_content: &[u8],
    page_num: u32,
    page_box: PageBox,
    fonts: &BTreeMap<Vec<u8>, &Dictionary>,
    xobjects: &HashMap<Vec<u8>, &lopdf::Stream>,
    initial_ctm: Matrix,
    depth: usize,
    out: &mut Vec<PdfTextSpan>,
) {
    if depth > 8 {
        return;
    }
    let Ok(content) = Content::decode(raw_content) else {
        return;
    };

    let mut gs = GraphicsState {
        ctm: initial_ctm,
        ..GraphicsState::default()
    };
    let mut gs_stack: Vec<GraphicsState> = Vec::new();
    let mut in_text = false;
    let mut tm = Matrix::identity();
    let mut tlm = Matrix::identity();

    for op in &content.operations {
        match op.operator.as_str() {
            "q" => gs_stack.push(gs.clone()),
            "Q" => {
                if let Some(prev) = gs_stack.pop() {
                    gs = prev;
                }
            }
            "cm" if op.operands.len() >= 6 => {
                if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) = (
                    obj_as_f32(&op.operands[0]),
                    obj_as_f32(&op.operands[1]),
                    obj_as_f32(&op.operands[2]),
                    obj_as_f32(&op.operands[3]),
                    obj_as_f32(&op.operands[4]),
                    obj_as_f32(&op.operands[5]),
                ) {
                    gs.ctm = Matrix::mul(Matrix([a, b, c, d, e, f]), gs.ctm);
                }
            }
            "BT" => {
                in_text = true;
                tm = Matrix::identity();
                tlm = Matrix::identity();
            }
            "ET" => {
                in_text = false;
            }
            "Tf" if op.operands.len() >= 2 => {
                if let Ok(name) = op.operands[0].as_name() {
                    gs.font_name = name.to_vec();
                }
                if let Some(sz) = obj_as_f32(&op.operands[1]) {
                    gs.font_size = sz.abs().max(0.1);
                }
            }
            "Tc" if !op.operands.is_empty() => {
                if let Some(v) = obj_as_f32(&op.operands[0]) {
                    gs.char_space = v;
                }
            }
            "Tw" if !op.operands.is_empty() => {
                if let Some(v) = obj_as_f32(&op.operands[0]) {
                    gs.word_space = v;
                }
            }
            "Tz" if !op.operands.is_empty() => {
                if let Some(v) = obj_as_f32(&op.operands[0]) {
                    gs.horiz_scale = v;
                }
            }
            "TL" if !op.operands.is_empty() => {
                if let Some(v) = obj_as_f32(&op.operands[0]) {
                    gs.text_leading = v;
                }
            }
            "Ts" if !op.operands.is_empty() => {
                if let Some(v) = obj_as_f32(&op.operands[0]) {
                    gs.text_rise = v;
                }
            }
            "Td" if in_text && op.operands.len() >= 2 => {
                if let (Some(tx), Some(ty)) =
                    (obj_as_f32(&op.operands[0]), obj_as_f32(&op.operands[1]))
                {
                    tlm = Matrix::mul(Matrix([1.0, 0.0, 0.0, 1.0, tx, ty]), tlm);
                    tm = tlm;
                }
            }
            "TD" if in_text && op.operands.len() >= 2 => {
                if let (Some(tx), Some(ty)) =
                    (obj_as_f32(&op.operands[0]), obj_as_f32(&op.operands[1]))
                {
                    gs.text_leading = -ty;
                    tlm = Matrix::mul(Matrix([1.0, 0.0, 0.0, 1.0, tx, ty]), tlm);
                    tm = tlm;
                }
            }
            "Tm" if in_text && op.operands.len() >= 6 => {
                if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) = (
                    obj_as_f32(&op.operands[0]),
                    obj_as_f32(&op.operands[1]),
                    obj_as_f32(&op.operands[2]),
                    obj_as_f32(&op.operands[3]),
                    obj_as_f32(&op.operands[4]),
                    obj_as_f32(&op.operands[5]),
                ) {
                    tm = Matrix([a, b, c, d, e, f]);
                    tlm = tm;
                }
            }
            "T*" if in_text => {
                tlm = Matrix::mul(Matrix([1.0, 0.0, 0.0, 1.0, 0.0, -gs.text_leading]), tlm);
                tm = tlm;
            }
            "Tj" | "'" | "\"" if in_text && !op.operands.is_empty() => {
                if op.operator == "'" || op.operator == "\"" {
                    if op.operator == "\"" && op.operands.len() >= 3 {
                        if let Some(aw) = obj_as_f32(&op.operands[0]) {
                            gs.word_space = aw;
                        }
                        if let Some(ac) = obj_as_f32(&op.operands[1]) {
                            gs.char_space = ac;
                        }
                    }
                    tlm = Matrix::mul(Matrix([1.0, 0.0, 0.0, 1.0, 0.0, -gs.text_leading]), tlm);
                    tm = tlm;
                }
                if let Some(bytes) = op.operands.last().and_then(|o| o.as_str().ok()) {
                    emit_text_chunk(
                        doc,
                        page_num,
                        page_box,
                        fonts.get(&gs.font_name).copied(),
                        &gs,
                        &mut tm,
                        bytes,
                        out,
                    );
                }
            }
            "TJ" if in_text && !op.operands.is_empty() => {
                if let Ok(arr) = op.operands[0].as_array() {
                    emit_tj_array(
                        doc,
                        page_num,
                        page_box,
                        fonts.get(&gs.font_name).copied(),
                        &gs,
                        &mut tm,
                        arr,
                        out,
                    );
                }
            }
            "Do" if !op.operands.is_empty() => {
                if let Ok(name) = op.operands[0].as_name()
                    && let Some(stream) = xobjects.get(name)
                    && stream
                        .dict
                        .get(b"Subtype")
                        .and_then(Object::as_name)
                        .is_ok_and(|s| s == b"Form")
                    && let Ok(bytes) = stream.decompressed_content()
                {
                    let form_matrix = stream
                        .dict
                        .get(b"Matrix")
                        .and_then(Object::as_array)
                        .ok()
                        .and_then(|arr| {
                            if arr.len() >= 6 {
                                Some(Matrix([
                                    obj_as_f32(&arr[0])?,
                                    obj_as_f32(&arr[1])?,
                                    obj_as_f32(&arr[2])?,
                                    obj_as_f32(&arr[3])?,
                                    obj_as_f32(&arr[4])?,
                                    obj_as_f32(&arr[5])?,
                                ]))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(Matrix::identity);

                    let mut sub_fonts = fonts.clone();
                    let mut sub_xobjs = xobjects.clone();
                    if let Ok(res_obj) = stream.dict.get(b"Resources")
                        && let Some(res_dict) = resolve_dict(doc, res_obj)
                    {
                        collect_fonts_from_resources(doc, res_dict, &mut sub_fonts);
                        collect_xobjects_from_resources(doc, res_dict, &mut sub_xobjs);
                    }
                    let combined_ctm = Matrix::mul(form_matrix, gs.ctm);
                    extract_stream_spans(
                        doc,
                        &bytes,
                        page_num,
                        page_box,
                        &sub_fonts,
                        &sub_xobjs,
                        combined_ctm,
                        depth + 1,
                        out,
                    );
                }
            }
            _ => {}
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_text_chunk(
    doc: &Document,
    page_num: u32,
    page_box: PageBox,
    font_dict: Option<&Dictionary>,
    gs: &GraphicsState,
    tm: &mut Matrix,
    bytes: &[u8],
    out: &mut Vec<PdfTextSpan>,
) {
    let decoded = decode_pdf_bytes(doc, font_dict, bytes);
    let trimmed = decoded.trim();
    let trm = Matrix::mul(*tm, gs.ctm);
    let (px, py) = trm.transform_point(0.0, gs.text_rise);
    let (top_x, top_y) = page_box.to_top_down(px, py);
    let eff_size = (gs.font_size * trm.scale_y()).clamp(2.0, 200.0);
    let char_count = decoded.chars().count().max(1) as f32;
    let est_w = (char_count * eff_size * 0.52 * (gs.horiz_scale / 100.0)).max(2.0);

    // Advance text matrix in text units
    let tx_advance =
        (char_count * gs.font_size * 0.52 + char_count * gs.char_space) * (gs.horiz_scale / 100.0);
    *tm = Matrix::mul(Matrix([1.0, 0.0, 0.0, 1.0, tx_advance, 0.0]), *tm);

    if !trimmed.is_empty() {
        out.push(PdfTextSpan {
            page: page_num,
            x: top_x,
            y: top_y,
            width: est_w,
            font_size: eff_size,
            text: trimmed.to_string(),
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_tj_array(
    doc: &Document,
    page_num: u32,
    page_box: PageBox,
    font_dict: Option<&Dictionary>,
    gs: &GraphicsState,
    tm: &mut Matrix,
    arr: &[Object],
    out: &mut Vec<PdfTextSpan>,
) {
    let mut buf = String::new();
    let mut start_x = 0.0f32;
    let mut start_y = 0.0f32;
    let mut start_size = gs.font_size;
    let mut has_start = false;

    let flush = |buf: &mut String,
                 has_start: &mut bool,
                 start_x: f32,
                 start_y: f32,
                 start_size: f32,
                 out: &mut Vec<PdfTextSpan>| {
        let t = buf.trim();
        if !t.is_empty() {
            let char_count = t.chars().count().max(1) as f32;
            out.push(PdfTextSpan {
                page: page_num,
                x: start_x,
                y: start_y,
                width: (char_count * start_size * 0.52).max(2.0),
                font_size: start_size,
                text: t.to_string(),
            });
        }
        buf.clear();
        *has_start = false;
    };

    for item in arr {
        match item {
            Object::String(bytes, _) => {
                let piece = decode_pdf_bytes(doc, font_dict, bytes);
                let trm = Matrix::mul(*tm, gs.ctm);
                let (px, py) = trm.transform_point(0.0, gs.text_rise);
                let (top_x, top_y) = page_box.to_top_down(px, py);
                let eff_size = (gs.font_size * trm.scale_y()).clamp(2.0, 200.0);

                if !has_start {
                    start_x = top_x;
                    start_y = top_y;
                    start_size = eff_size;
                    has_start = true;
                }
                buf.push_str(&piece);

                let chars = piece.chars().count() as f32;
                let adv = (chars * gs.font_size * 0.52 + chars * gs.char_space)
                    * (gs.horiz_scale / 100.0);
                *tm = Matrix::mul(Matrix([1.0, 0.0, 0.0, 1.0, adv, 0.0]), *tm);
            }
            Object::Real(_) | Object::Integer(_) => {
                if let Some(kern) = obj_as_f32(item) {
                    let shift = (-kern / 1000.0) * gs.font_size * (gs.horiz_scale / 100.0);
                    *tm = Matrix::mul(Matrix([1.0, 0.0, 0.0, 1.0, shift, 0.0]), *tm);
                    if kern <= -650.0 {
                        // Wide kerning jump often separates adjacent table columns
                        flush(
                            &mut buf,
                            &mut has_start,
                            start_x,
                            start_y,
                            start_size,
                            out,
                        );
                    } else if kern <= -180.0 && !buf.ends_with(' ') && !buf.is_empty() {
                        buf.push(' ');
                    }
                }
            }
            _ => {}
        }
    }

    flush(
        &mut buf,
        &mut has_start,
        start_x,
        start_y,
        start_size,
        out,
    );
}

/// Extracts all positioned text spans from an in-memory PDF document.
pub fn extract_pdf_text_spans(pdf_bytes: &[u8]) -> Result<Vec<PdfTextSpan>, String> {
    let doc = Document::load_mem(pdf_bytes)
        .map_err(|e| format!("PDF konnte nicht geladen werden: {e}"))?;
    let pages = doc.get_pages();
    if pages.is_empty() {
        return Err("PDF enthält keine Seiten.".to_string());
    }

    let mut spans = Vec::new();
    for (page_num, page_id) in pages {
        let page_box = get_page_box_and_rotation(&doc, page_id);
        let mut fonts = BTreeMap::new();
        let mut xobjects = HashMap::new();
        if let Ok((res_opt, res_ids)) = doc.get_page_resources(page_id) {
            if let Some(res) = res_opt {
                collect_fonts_from_resources(&doc, res, &mut fonts);
                collect_xobjects_from_resources(&doc, res, &mut xobjects);
            }
            for id in res_ids {
                if let Ok(res) = doc.get_dictionary(id) {
                    collect_fonts_from_resources(&doc, res, &mut fonts);
                    collect_xobjects_from_resources(&doc, res, &mut xobjects);
                }
            }
        }

        let content_bytes = doc.get_page_content(page_id);
        if !content_bytes.is_empty() {
            extract_stream_spans(
                &doc,
                &content_bytes,
                page_num,
                page_box,
                &fonts,
                &xobjects,
                Matrix::identity(),
                0,
                &mut spans,
            );
        }
    }

    Ok(spans)
}

use std::collections::HashSet;
use std::sync::OnceLock;
use regex::Regex;

#[derive(Debug, Clone)]
struct VisualRow {
    y: f32,
    spans: Vec<PdfTextSpan>,
}

#[derive(Debug, Clone)]
struct ColumnLayout {
    /// Sorted left-to-right column descriptors: `(col_index 0..=5, anchor_x, header_width)`.
    /// Column indices: 0=BMK, 1=Qty, 2=Desc, 3=Type, 4=Supplier, 5=Article.
    columns: Vec<(usize, f32, f32)>,
    /// Right boundary for each entry in `columns` (midpoint to next column; last is +inf).
    right_bounds: Vec<f32>,
}

impl ColumnLayout {
    fn from_anchors(anchors: [Option<(f32, f32)>; 6]) -> Option<Self> {
        let mut cols: Vec<(usize, f32, f32)> = anchors
            .iter()
            .enumerate()
            .filter_map(|(idx, xw)| xw.map(|(x, w)| (idx, x, w)))
            .collect();
        if cols.len() < 3 {
            return None;
        }
        // Require at least BMK or Article/Type/Qty alongside other columns to avoid false 3-word matches
        let has_key_col = anchors[0].is_some()
            || anchors[5].is_some()
            || anchors[3].is_some()
            || anchors[1].is_some();
        if !has_key_col {
            return None;
        }
        cols.sort_by(|a, b| a.1.total_cmp(&b.1));
        let mut right_bounds = Vec::with_capacity(cols.len());
        for w in cols.windows(2) {
            right_bounds.push((w[0].1 + w[1].1) * 0.5);
        }
        right_bounds.push(f32::INFINITY);
        Some(Self {
            columns: cols,
            right_bounds,
        })
    }

    fn assign_column(&self, span_x: f32, span_w: f32) -> usize {
        // First check if the span sits directly inside a header column's horizontal footprint
        // (handles right-aligned Quantity values under a narrow "Menge" header).
        let center_x = span_x + span_w * 0.5;
        for (i, &(col_idx, ax, aw)) in self.columns.iter().enumerate() {
            let next_ax = self
                .columns
                .get(i + 1)
                .map(|c| c.1 - 2.0)
                .unwrap_or(f32::INFINITY);
            let col_right = (ax + aw + 6.0).min(next_ax);
            if span_x >= ax - 6.0 && center_x <= col_right && span_x < next_ax {
                return col_idx;
            }
        }
        for (i, &bound) in self.right_bounds.iter().enumerate() {
            if span_x < bound {
                return self.columns[i].0;
            }
        }
        self.columns.last().map(|c| c.0).unwrap_or(0)
    }
}

fn norm_header_token(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn match_header_col(text: &str, rules: &CompiledImportRules) -> Option<usize> {
    let trimmed = text.trim();
    if trimmed.is_empty() || trimmed.len() > 56 {
        return None;
    }
    let norm = norm_header_token(trimmed);
    if norm.len() < 2 || norm.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let groups = [
        &rules.raw.header_aliases.bmk,
        &rules.raw.header_aliases.quantity,
        &rules.raw.header_aliases.description,
        &rules.raw.header_aliases.type_number,
        &rules.raw.header_aliases.supplier,
        &rules.raw.header_aliases.article_number,
    ];

    // Pass 1: exact normalized match (ensures "Bezeichnung 1" matches Type No., not Description)
    for (idx, aliases) in groups.iter().enumerate() {
        if aliases.iter().any(|a| norm_header_token(a) == norm) {
            return Some(idx);
        }
    }

    // Pass 2: split by delimiters ('/', '(', '[', ':', '\n') and check exact part match (longest alias wins)
    let mut best: Option<(usize, usize)> = None;
    for part in trimmed.split(['/', '(', '[', ':', '\n']) {
        let pnorm = norm_header_token(part);
        if pnorm.len() < 2 {
            continue;
        }
        for (idx, aliases) in groups.iter().enumerate() {
            for a in *aliases {
                let anorm = norm_header_token(a);
                if anorm == pnorm
                    && best.is_none_or(|(_, best_len)| anorm.len() > best_len)
                {
                    best = Some((idx, anorm.len()));
                }
            }
        }
    }
    if let Some((idx, _)) = best {
        return Some(idx);
    }

    // Pass 3: prefix match for longer aliases (>= 5 chars) like "Menge [Stk]" or "Artikelnummer (ERP)"
    for (idx, aliases) in groups.iter().enumerate() {
        for a in *aliases {
            let anorm = norm_header_token(a);
            if anorm.len() >= 5
                && norm.starts_with(&anorm)
                && best.is_none_or(|(_, best_len)| anorm.len() > best_len)
            {
                best = Some((idx, anorm.len()));
            }
        }
    }
    best.map(|(idx, _)| idx)
}

fn cluster_spans_into_rows(spans: &[PdfTextSpan]) -> Vec<VisualRow> {
    let mut sorted: Vec<PdfTextSpan> = spans.to_vec();
    sorted.sort_by(|a, b| a.y.total_cmp(&b.y).then_with(|| a.x.total_cmp(&b.x)));

    let mut rows: Vec<VisualRow> = Vec::new();
    for sp in sorted {
        let tol = (sp.font_size * 0.42).clamp(2.5, 5.5);
        if let Some(last) = rows.last_mut()
            && (sp.y - last.y).abs() <= tol
        {
            last.spans.push(sp);
            let sum_y: f32 = last.spans.iter().map(|s| s.y).sum();
            last.y = sum_y / (last.spans.len() as f32);
            continue;
        }
        rows.push(VisualRow {
            y: sp.y,
            spans: vec![sp],
        });
    }
    for r in &mut rows {
        r.spans.sort_by(|a, b| a.x.total_cmp(&b.x));
    }
    rows
}

fn row_full_line(row: &VisualRow) -> String {
    row.spans
        .iter()
        .map(|s| s.text.trim())
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_unit_word(tok: &str) -> bool {
    let clean = tok
        .trim()
        .trim_end_matches('.')
        .trim_end_matches(',')
        .to_lowercase();
    matches!(
        clean.as_str(),
        "stk" | "st" | "stück" | "stueck" | "pcs" | "pc" | "x" | "×" | "mal" | "m"
    )
}

/// Parses a plausible BOM quantity token (`1..=9999`), avoiding 6..10-digit article numbers.
fn parse_qty_token(tok: &str) -> Option<usize> {
    let trimmed = tok
        .trim()
        .trim_end_matches(['x', '×', 'X'])
        .trim();
    if trimmed.is_empty() {
        return None;
    }
    // Reject tokens with hyphens/slashes/dots-between-digits-beyond-decimals (like "285-135" or "1.2.3")
    if trimmed.contains('-') || trimmed.contains('/') {
        return None;
    }
    let normalized = trimmed.replace(',', ".");
    if let Ok(v) = normalized.parse::<f64>()
        && (1.0..=9999.0).contains(&v)
        && (v - v.round()).abs() < 1e-4
    {
        return Some(v.round() as usize);
    }
    None
}

fn is_definite_article_token(tok: &str) -> bool {
    static VENDOR_PREF_RE: OnceLock<Regex> = OnceLock::new();
    static HYPHEN_CODE_RE: OnceLock<Regex> = OnceLock::new();
    static LONG_NUM_RE: OnceLock<Regex> = OnceLock::new();

    let vendor_re = VENDOR_PREF_RE
        .get_or_init(|| Regex::new(r"^[A-Za-z]{2,5}[.:][A-Za-z0-9._/-]+$").expect("valid regex"));
    let hyphen_re = HYPHEN_CODE_RE
        .get_or_init(|| Regex::new(r"^\d{3,4}-\d{3,4}(?:/\d{3}-\d{3})?$").expect("valid regex"));
    let long_num_re =
        LONG_NUM_RE.get_or_init(|| Regex::new(r"^\d{6,12}$").expect("valid regex"));

    let t = tok.trim();
    vendor_re.is_match(t) || hyphen_re.is_match(t) || long_num_re.is_match(t)
}

fn parse_quantity_cell(s: &str) -> Option<usize> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut tokens = trimmed.split_whitespace();
    let first = tokens.next()?;
    if let Some(q) = parse_qty_token(first) {
        return Some(q);
    }
    None
}

/// Extracts `(quantity, article_number)` across potentially merged, swapped, or inverted cells
/// (e.g., `"6 PXC.3213946"`, `"PXC.3213946 6"`, `"6\nPXC.3213946"`, `"6 Stk"`).
fn extract_qty_and_article(qty_cell: &str, art_cell: &str) -> (Option<usize>, String) {
    let q_trim = qty_cell.trim();
    let a_trim = art_cell.trim();

    // Case 1: Both cells non-empty
    if !q_trim.is_empty() && !a_trim.is_empty() {
        if let Some(q) = parse_quantity_cell(q_trim) {
            return (Some(q), a_trim.to_string());
        }
        // Handle completely swapped qty & article columns
        if is_definite_article_token(q_trim)
            && let Some(q) = parse_quantity_cell(a_trim)
        {
            return (Some(q), q_trim.to_string());
        }
        return (None, a_trim.to_string());
    }

    // Case 2: Only qty_cell is non-empty (might contain merged "6 PXC.3213946" or "PXC.3213946 6")
    if !q_trim.is_empty() && a_trim.is_empty() {
        let tokens: Vec<&str> = q_trim.split_whitespace().collect();
        if tokens.len() == 1 {
            if let Some(q) = parse_qty_token(tokens[0]) {
                return (Some(q), String::new());
            }
            if is_definite_article_token(tokens[0]) {
                return (None, tokens[0].to_string());
            }
            return (parse_quantity_cell(q_trim), String::new());
        }

        let mut found_qty: Option<usize> = None;
        let mut art_parts: Vec<&str> = Vec::new();
        for tok in &tokens {
            if is_unit_word(tok) {
                continue;
            }
            if found_qty.is_none()
                && !is_definite_article_token(tok)
                && let Some(q) = parse_qty_token(tok)
            {
                found_qty = Some(q);
                continue;
            }
            art_parts.push(tok);
        }
        if let Some(q) = found_qty {
            return (Some(q), art_parts.join(" "));
        }
        if tokens.iter().any(|t| is_definite_article_token(t)) {
            return (None, q_trim.to_string());
        }
        return (parse_quantity_cell(q_trim), String::new());
    }

    // Case 3: Only art_cell is non-empty (might contain merged "6 PXC.3213946", "PXC.3213946 6", "6 3213946", or wrapped "4")
    if q_trim.is_empty() && !a_trim.is_empty() {
        let tokens: Vec<&str> = a_trim.split_whitespace().collect();
        if tokens.len() == 1
            && !is_definite_article_token(tokens[0])
            && let Some(q) = parse_qty_token(tokens[0])
        {
            return (Some(q), String::new());
        }
        if tokens.len() >= 2 && tokens.iter().any(|t| is_definite_article_token(t)) {
            let mut found_qty: Option<usize> = None;
            let mut art_parts: Vec<&str> = Vec::new();
            for tok in &tokens {
                if is_unit_word(tok) {
                    continue;
                }
                if found_qty.is_none()
                    && !is_definite_article_token(tok)
                    && let Some(q) = parse_qty_token(tok)
                {
                    found_qty = Some(q);
                    continue;
                }
                art_parts.push(tok);
            }
            if let Some(q) = found_qty
                && !art_parts.is_empty()
            {
                return (Some(q), art_parts.join(" "));
            }
        }
        return (None, a_trim.to_string());
    }

    (None, String::new())
}

#[derive(Debug, Clone)]
struct PartialBomRow {
    page: u32,
    bmk: String,
    quantity: Option<usize>,
    description: String,
    type_number: String,
    supplier: String,
    article_number: String,
    last_y: f32,
}

impl PartialBomRow {
    fn has_payload(&self) -> bool {
        !self.article_number.is_empty()
            || !self.type_number.is_empty()
            || !self.description.is_empty()
    }

    fn is_complete(&self) -> bool {
        self.quantity.is_some() && self.has_payload()
    }

    fn into_raw(self) -> Option<RawBomRow> {
        let qty = self.quantity?;
        if !self.has_payload() {
            return None;
        }
        Some(RawBomRow {
            page: self.page,
            bmk: self.bmk,
            quantity: qty,
            description: self.description,
            type_number: self.type_number,
            supplier: self.supplier,
            article_number: self.article_number,
        })
    }
}

fn append_spaced(dst: &mut String, piece: &str) {
    let p = piece.trim();
    if p.is_empty() {
        return;
    }
    if !dst.is_empty() {
        dst.push(' ');
    }
    dst.push_str(p);
}

fn match_header_col_exact(text: &str, rules: &CompiledImportRules) -> Option<usize> {
    let norm = norm_header_token(text);
    if norm.len() < 2 || norm.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let groups = [
        &rules.raw.header_aliases.bmk,
        &rules.raw.header_aliases.quantity,
        &rules.raw.header_aliases.description,
        &rules.raw.header_aliases.type_number,
        &rules.raw.header_aliases.supplier,
        &rules.raw.header_aliases.article_number,
    ];
    for (idx, aliases) in groups.iter().enumerate() {
        if aliases.iter().any(|a| norm_header_token(a) == norm) {
            return Some(idx);
        }
    }
    None
}

struct DetectedHeader {
    row_idx: usize,
    y: f32,
    layout: ColumnLayout,
    consumed_spans: HashSet<(usize, usize)>,
}

fn detect_page_header(
    rows: &[VisualRow],
    rules: &CompiledImportRules,
) -> Option<DetectedHeader> {
    for (idx, row) in rows.iter().enumerate() {
        let line = row_full_line(row);
        if rules.is_footer_line(&line) {
            continue;
        }

        let mut anchors: [Option<(f32, f32)>; 6] = [None; 6];
        let mut anchor_lens: [usize; 6] = [0; 6];
        let mut consumed_spans: HashSet<(usize, usize)> = HashSet::new();

        for (s_idx, sp) in row.spans.iter().enumerate() {
            if let Some(col) = match_header_col(&sp.text, rules) {
                let len = norm_header_token(&sp.text).len();
                if anchors[col].is_none() || len > anchor_lens[col] {
                    anchors[col] = Some((sp.x, sp.width));
                    anchor_lens[col] = len;
                }
                consumed_spans.insert((idx, s_idx));
            }
        }

        // Check vertically wrapped/hyphenated header continuations (e.g. "Betriebsmittel-\nkennzeichen", "Bezeichnung\n1")
        if let Some(next_row) = rows.get(idx + 1)
            && next_row.y - row.y <= 16.0
        {
            for sp1 in &row.spans {
                let t1 = sp1.text.trim();
                if t1.is_empty() {
                    continue;
                }
                for (s2_idx, sp2) in next_row.spans.iter().enumerate() {
                    let t2 = sp2.text.trim();
                    if t2.is_empty() {
                        continue;
                    }
                    let x_close = (sp2.x - sp1.x).abs() <= 28.0
                        || (sp2.x >= sp1.x - 10.0 && sp2.x <= sp1.x + sp1.width + 10.0);
                    if !x_close {
                        continue;
                    }
                    let combined = if t1.ends_with('-') {
                        format!("{}{}", t1.trim_end_matches('-'), t2)
                    } else {
                        format!("{t1} {t2}")
                    };
                    if let Some(col) = match_header_col_exact(&combined, rules) {
                        let comb_len = norm_header_token(&combined).len();
                        if anchors[col].is_none() || comb_len > anchor_lens[col] {
                            anchors[col] = Some((sp1.x.min(sp2.x), sp1.width.max(sp2.width)));
                            anchor_lens[col] = comb_len;
                        }
                        consumed_spans.insert((idx + 1, s2_idx));
                    }
                }
            }
        }

        let matched = anchors.iter().filter(|a| a.is_some()).count();
        if matched >= 3 {
            // Peek nearby rows for EPLAN's slipped column headers (e.g. "Typnummer" sitting in data row 1)
            for (n_idx, neighbor) in rows.iter().enumerate() {
                if n_idx != idx && (neighbor.y - row.y).abs() <= 24.0 {
                    for (s_idx, sp) in neighbor.spans.iter().enumerate() {
                        if let Some(col) = match_header_col(&sp.text, rules)
                            && anchors[col].is_none()
                        {
                            anchors[col] = Some((sp.x, sp.width));
                            consumed_spans.insert((n_idx, s_idx));
                        }
                    }
                }
            }
            if let Some(layout) = ColumnLayout::from_anchors(anchors) {
                return Some(DetectedHeader {
                    row_idx: idx,
                    y: row.y,
                    layout,
                    consumed_spans,
                });
            }
        }
    }
    None
}

/// Parses positioned PDF text spans into normalized BOM rows (`RawBomRow`),
/// resilient to variable column orders, hyphenated headers, slipped headers (`Typnummer`),
/// merged/inverted Quantity+Article cells, multi-line cells, page breaks, and EPLAN/CAD footers.
pub fn parse_eplan_bom_tables(
    spans: &[PdfTextSpan],
    rules: &CompiledImportRules,
) -> Vec<RawBomRow> {
    let mut by_page: BTreeMap<u32, Vec<PdfTextSpan>> = BTreeMap::new();
    for sp in spans {
        by_page.entry(sp.page).or_default().push(sp.clone());
    }

    let mut out_rows: Vec<RawBomRow> = Vec::new();
    let mut active_layout: Option<ColumnLayout> = None;
    let mut last_bmk_context = String::new();

    for (page, page_spans) in by_page {
        let rows = cluster_spans_into_rows(&page_spans);
        if rows.is_empty() {
            continue;
        }

        let mut header_row_idx: Option<usize> = None;
        let mut header_y: Option<f32> = None;
        let mut consumed_header_spans: HashSet<(usize, usize)> = HashSet::new();

        if let Some(hdr) = detect_page_header(&rows, rules) {
            active_layout = Some(hdr.layout);
            header_row_idx = Some(hdr.row_idx);
            header_y = Some(hdr.y);
            consumed_header_spans = hdr.consumed_spans;
        }

        let Some(layout) = active_layout.as_ref() else {
            continue;
        };

        let start_idx = header_row_idx.map(|i| i + 1).unwrap_or(0);
        let mut pending: Option<PartialBomRow> = None;

        for (r_idx, row) in rows.iter().enumerate().skip(start_idx) {
            let full_line = row_full_line(row);
            if rules.is_footer_line(&full_line) {
                continue;
            }

            let mut cols: [Vec<String>; 6] = Default::default();
            let near_header = header_y.is_some_and(|hy| (row.y - hy).abs() <= 28.0);
            for (s_idx, sp) in row.spans.iter().enumerate() {
                if consumed_header_spans.contains(&(r_idx, s_idx)) {
                    continue;
                }
                let col_idx = layout.assign_column(sp.x, sp.width);
                if near_header && match_header_col(&sp.text, rules) == Some(col_idx) {
                    continue;
                }
                let txt = sp.text.trim();
                if !txt.is_empty() {
                    cols[col_idx].push(txt.to_string());
                }
            }

            let bmk_raw = cols[0].join(" ");
            let bmk_cell = CompiledImportRules::normalize_bmk(&bmk_raw);
            let qty_cell = cols[1].join(" ").trim().to_string();
            let desc_cell = cols[2].join(" ").trim().to_string();
            let type_cell = cols[3].join(" ").trim().to_string();
            let sup_cell = cols[4].join(" ").trim().to_string();
            let art_cell = cols[5].join(" ").trim().to_string();

            if bmk_cell.is_empty()
                && qty_cell.is_empty()
                && desc_cell.is_empty()
                && type_cell.is_empty()
                && sup_cell.is_empty()
                && art_cell.is_empty()
            {
                continue;
            }

            let (qty_opt, norm_art) = extract_qty_and_article(&qty_cell, &art_cell);

            if !bmk_cell.is_empty() {
                last_bmk_context = bmk_cell.clone();
            }

            // Can this visual line merge into an open `pending` row on the same page?
            if let Some(prev) = pending.as_mut() {
                let close_vertically = (row.y - prev.last_y).abs() <= 28.0;
                // Case A: Previous line had article/desc/type without quantity (e.g. "PXC.3213946\n6")
                if prev.quantity.is_none()
                    && close_vertically
                    && bmk_cell.is_empty()
                    && qty_opt.is_some()
                    && norm_art.is_empty()
                {
                    prev.quantity = qty_opt;
                    append_spaced(&mut prev.description, &desc_cell);
                    append_spaced(&mut prev.type_number, &type_cell);
                    if prev.supplier.is_empty() && !sup_cell.is_empty() {
                        prev.supplier = sup_cell;
                    }
                    prev.last_y = row.y;
                    continue;
                }
                // Case B: Previous line had BMK + quantity without article/type/desc (e.g. "6\nPXC.3213946")
                if prev.quantity.is_some()
                    && !prev.has_payload()
                    && close_vertically
                    && bmk_cell.is_empty()
                    && qty_opt.is_none()
                    && (!norm_art.is_empty() || !type_cell.is_empty() || !desc_cell.is_empty())
                {
                    if !norm_art.is_empty() {
                        prev.article_number = norm_art;
                    }
                    append_spaced(&mut prev.description, &desc_cell);
                    append_spaced(&mut prev.type_number, &type_cell);
                    if prev.supplier.is_empty() && !sup_cell.is_empty() {
                        prev.supplier = sup_cell;
                    }
                    prev.last_y = row.y;
                    continue;
                }
                // Case C: Previous line is complete; this line is a wrapped description/type continuation
                if prev.is_complete()
                    && bmk_cell.is_empty()
                    && qty_opt.is_none()
                    && norm_art.is_empty()
                    && sup_cell.is_empty()
                    && (!desc_cell.is_empty() || !type_cell.is_empty())
                {
                    append_spaced(&mut prev.description, &desc_cell);
                    append_spaced(&mut prev.type_number, &type_cell);
                    prev.last_y = row.y;
                    continue;
                }
            }

            // Start a new logical row (flushing any completed previous partial row)
            if let Some(prev) = pending.take()
                && let Some(done) = prev.into_raw()
            {
                out_rows.push(done);
            }

            let effective_bmk = if !bmk_cell.is_empty() {
                bmk_cell
            } else {
                last_bmk_context.clone()
            };

            // Only start a new PartialBomRow if this line carries item data (qty, article, type, or desc)
            if qty_opt.is_some()
                || !norm_art.is_empty()
                || !type_cell.is_empty()
                || !desc_cell.is_empty()
            {
                pending = Some(PartialBomRow {
                    page,
                    bmk: effective_bmk,
                    quantity: qty_opt,
                    description: desc_cell,
                    type_number: type_cell,
                    supplier: sup_cell,
                    article_number: norm_art,
                    last_y: row.y,
                });
            }
        }

        if let Some(prev) = pending.take()
            && let Some(done) = prev.into_raw()
        {
            out_rows.push(done);
        }
    }

    out_rows
}

