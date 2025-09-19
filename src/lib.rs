use wasm_bindgen::prelude::*;
use image::{ImageOutputFormat, DynamicImage};
use std::io::Cursor;

#[wasm_bindgen]
pub fn convert(data: &[u8], format: Option<String>, quality: Option<f32>) -> Vec<u8> {
    // GIF 파일인지 확인하고 첫 번째 프레임만 추출
    let img = if is_gif(data) {
        extract_first_frame(data)
    } else {
        image::load_from_memory(data).expect("Failed to decode image")
    };

    // 기본값: webp, quality 0.8
    let target_format = format.unwrap_or_else(|| "webp".to_string()).to_lowercase();
    let q = quality.unwrap_or(0.8);
    let mut buf = Cursor::new(Vec::new());

    match target_format.as_str() {
        "jpeg" | "jpg" => {
            let q_u8 = (q.clamp(0.0, 1.0) * 100.0) as u8;
            img.write_to(&mut buf, ImageOutputFormat::Jpeg(q_u8))
                .expect("JPEG encode failed");
        }
        "png" => {
            // PNG도 JPEG를 통한 압축 방식 사용
            let quality = q.clamp(0.0, 1.0);
            return encode_png_via_jpeg(&img, quality);
        }
        "webp" => {
            // JPEG로 먼저 압축한 후 WebP로 변환하여 압축률 향상
            let quality = q.clamp(0.0, 1.0);
            return encode_webp_via_jpeg(&img, quality);
        }
        _ => panic!("Unsupported format"),
    }

    buf.into_inner()
}

// JPEG를 통한 WebP 압축 함수
fn encode_webp_via_jpeg(img: &DynamicImage, quality: f32) -> Vec<u8> {
    // 1단계: JPEG로 압축 (품질에 따라 압축률 조정)
    let jpeg_quality = (quality * 100.0) as u8;
    let mut jpeg_buf = Cursor::new(Vec::new());
    
    img.write_to(&mut jpeg_buf, ImageOutputFormat::Jpeg(jpeg_quality))
        .expect("JPEG encode failed");
    
    // 2단계: JPEG 데이터를 다시 이미지로 디코딩
    let jpeg_data = jpeg_buf.into_inner();
    let compressed_img = image::load_from_memory(&jpeg_data)
        .expect("Failed to decode JPEG");
    
    // 3단계: 압축된 이미지를 WebP로 변환
    let mut webp_buf = Cursor::new(Vec::new());
    compressed_img.write_to(&mut webp_buf, ImageOutputFormat::WebP)
        .expect("WebP encode failed");
    
    webp_buf.into_inner()
}

// JPEG를 통한 PNG 압축 함수
fn encode_png_via_jpeg(img: &DynamicImage, quality: f32) -> Vec<u8> {
    // 1단계: JPEG로 압축
    let jpeg_quality = (quality * 100.0) as u8;
    let mut jpeg_buf = Cursor::new(Vec::new());
    
    img.write_to(&mut jpeg_buf, ImageOutputFormat::Jpeg(jpeg_quality))
        .expect("JPEG encode failed");
    
    // 2단계: JPEG 데이터를 다시 이미지로 디코딩
    let jpeg_data = jpeg_buf.into_inner();
    let compressed_img = image::load_from_memory(&jpeg_data)
        .expect("Failed to decode JPEG");
    
    // 3단계: 압축된 이미지를 PNG로 변환
    let mut png_buf = Cursor::new(Vec::new());
    compressed_img.write_to(&mut png_buf, ImageOutputFormat::Png)
        .expect("PNG encode failed");
    
    png_buf.into_inner()
}

// GIF 파일인지 확인하는 함수
fn is_gif(data: &[u8]) -> bool {
    if data.len() < 6 {
        return false;
    }
    
    // GIF 시그니처 확인 (GIF87a 또는 GIF89a)
    &data[0..6] == b"GIF87a" || &data[0..6] == b"GIF89a"
}

// GIF에서 첫 번째 프레임만 추출하는 함수
fn extract_first_frame(data: &[u8]) -> DynamicImage {
    // 일반적인 이미지 로딩으로 첫 번째 프레임을 가져옴
    // image 크레이트는 GIF의 첫 번째 프레임을 자동으로 반환함
    image::load_from_memory(data)
        .expect("Failed to decode GIF first frame")
}