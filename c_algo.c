int compute_trending_score(int base_score, int time_factor, int shares) {
    return base_score + (shares * 2) - (time_factor / 10);
}
