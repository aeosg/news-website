## 2026-10-13 - Optimizing Font Loading
**Learning:** Using `@import` for Google Fonts inside a `<style>` block blocks parallel downloads in HTML rendering.
**Action:** Always prefer `<link rel="preconnect">` and `<link rel="stylesheet">` directly in the `<head>` for faster First Contentful Paint.
