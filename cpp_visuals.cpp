extern "C" {
    int calculate_color_factor(int input, int max_val) {
        if (max_val == 0) return 0;
        int ratio = (input * 255) / max_val;
        if (ratio > 255) return 255;
        if (ratio < 0) return 0;
        return ratio;
    }
}
