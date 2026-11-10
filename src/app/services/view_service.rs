use lumina::core::blade::{BladeContext, BladeEngine};
use std::sync::OnceLock;

static BLADE: OnceLock<BladeEngine> = OnceLock::new();

pub struct ViewService;

impl ViewService {
    fn get_blade() -> &'static BladeEngine {
        BLADE.get_or_init(BladeEngine::new)
    }

    /// Render template Blade native Lumina v0.6.1
    pub fn render(template_name: &str, context: &BladeContext) -> String {
        Self::get_blade().render(template_name, context)
    }

    /// Alias untuk kompatibilitas perenderan template Blade
    pub fn render_blade(template_name: &str, context: &BladeContext) -> String {
        Self::render(template_name, context)
    }

    /// Format integer rupiah ke tampilan Indonesia: Rp25.000
    pub fn format_rupiah(amount: i64) -> String {
        let is_negative = amount < 0;
        let abs_val = amount.abs().to_string();
        let mut result = String::new();
        let chars: Vec<char> = abs_val.chars().collect();
        let len = chars.len();

        for (i, ch) in chars.iter().enumerate() {
            if i > 0 && (len - i).is_multiple_of(3) {
                result.push('.');
            }
            result.push(*ch);
        }

        if is_negative {
            format!("-Rp{}", result)
        } else {
            format!("Rp{}", result)
        }
    }
}
