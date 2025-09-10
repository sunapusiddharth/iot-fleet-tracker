use crate::ml_edge::types::{
    CargoTamperResult, DrowsinessResult, InferenceResult, LaneDepartureResult, LicensePlateResult,
};
use ndarray::ArrayView;

pub fn postprocess_output(
    output: &ArrayView<f32, ndarray::Ix1>,
    model_name: &str,
    threshold: f32,
) -> Result<InferenceResult, Box<dyn std::error::Error>> {
    match model_name {
        "drowsiness" => {
            // Assume output[0] = eye_closure_ratio, output[1] = is_drowsy_prob
            let eye_closure = output[0];
            let is_drowsy_prob = output[1];
            let is_drowsy = is_drowsy_prob > threshold;

            Ok(InferenceResult::Drowsiness(DrowsinessResult {
                is_drowsy,
                eye_closure_ratio: eye_closure,
                head_pose: (0.0, 0.0, 0.0), // Placeholder — add if model outputs
            }))
        }
        "lane_departure" => {
            // Assume output[0] = deviation_pixels, output[1] = lane_confidence
            let deviation = output[0] as i32;
            let confidence = output[1];
            let is_departing = deviation.abs() > 50 && confidence > threshold;

            Ok(InferenceResult::LaneDeparture(LaneDepartureResult {
                is_departing,
                deviation_pixels: deviation,
                lane_confidence: confidence,
            }))
        }
        "cargo_tamper" => {
            // Assume output[0] = motion_score, output[1] = object_count_change
            let motion_score = output[0];
            let object_change = output[1] as i32;
            let is_tampered = motion_score > threshold;

            Ok(InferenceResult::CargoTamper(CargoTamperResult {
                is_tampered,
                motion_score,
                object_count_change: object_change,
            }))
        }
        "license_plate" => {
            // This is simplified — real OCR needs character decoding
            let confidence = output[0];
            if confidence > threshold {
                Ok(InferenceResult::LicensePlate(LicensePlateResult {
                    plate_text: "ABC123".to_string(), // Placeholder
                    plate_confidence: confidence,
                    bounding_box: (0.1, 0.1, 0.2, 0.1), // Placeholder
                }))
            } else {
                Ok(InferenceResult::Unknown)
            }
        }
        _ => Ok(InferenceResult::Unknown),
    }
}
