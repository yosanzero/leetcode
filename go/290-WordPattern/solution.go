func wordPattern(pattern string, str string) bool {
    parts := strings.Split(str, " ")
    if len(pattern) != len(parts) {
        return false
    }
    m := make(map[byte]string)
    mr := make(map[string]byte)
    for i := 0; i < len(pattern); i++ {
        w, ok := m[pattern[i]]
        b, rok := mr[parts[i]]
        if !ok && !rok {
            m[pattern[i]] = parts[i]
            mr[parts[i]] = pattern[i]
            continue
        }
        if !ok || !rok {
            return false
        }
        if w != parts[i] || b != pattern[i] {
            return false
        }
    }
    return true
}
