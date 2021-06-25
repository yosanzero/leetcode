package number_match

import "testing"

func TestnumMatchingSubseq(t *testing.T) {
	if numMatchingSubseq("abcde", []string{"a", "bb", "acd", "ace"}) != 3 {
		t.Fatal("Example1")
	}
	if numMatchingSubseq("dsahjpjauf", []string{"ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"}) != 2 {
		t.Fatal("Example2")
	}

}
