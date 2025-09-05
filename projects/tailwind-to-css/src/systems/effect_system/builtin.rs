use super::*;

impl EffectSystem {
    pub fn builtin() -> Self {
        let mut new = Self::default();
        new.builtin_box_shadows();
        new.builtin_drop_shadows();
        new
    }
    fn builtin_box_shadows(&mut self) {
        // Default shadow (no modifier) maps to shadow-sm in v4
        self.set_box_shadow_default("0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)");
        // New v4 shadow scale entries
        self.insert_box_shadow("2xs", "0 1px rgb(0 0 0 / 0.05)");
        self.insert_box_shadow("xs", "0 1px 2px 0 rgb(0 0 0 / 0.05)");
        // Updated v4 shadow values
        self.insert_box_shadow("sm", "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)");
        self.insert_box_shadow("md", "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)");
        self.insert_box_shadow("lg", "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)");
        self.insert_box_shadow("xl", "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)");
        self.insert_box_shadow("2xl", "0 25px 50px -12px rgb(0 0 0 / 0.25)");
        self.insert_box_shadow("inner", "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)");
        self.insert_box_shadow("none", "0 0 #0000");
    }
    fn builtin_drop_shadows(&mut self) {
        // Default drop-shadow (no modifier) maps to drop-shadow-sm in v4
        self.set_drop_shadow_default("drop-shadow(0 1px 2px rgb(0 0 0 / 0.15))");
        // New v4 drop-shadow scale entries - note: no 2xs for drop-shadow in v4
        self.insert_drop_shadow("xs", "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))");
        // Updated v4 drop-shadow values
        self.insert_drop_shadow("sm", "drop-shadow(0 1px 2px rgb(0 0 0 / 0.15))");
        self.insert_drop_shadow("md", "drop-shadow(0 3px 3px rgb(0 0 0 / 0.12))");
        self.insert_drop_shadow("lg", "drop-shadow(0 4px 4px rgb(0 0 0 / 0.15))");
        self.insert_drop_shadow("xl", "drop-shadow(0 9px 7px rgb(0 0 0 / 0.1))");
        self.insert_drop_shadow("2xl", "drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))");
        self.insert_drop_shadow("none", "drop-shadow(0 0 #0000)");
    }
}
