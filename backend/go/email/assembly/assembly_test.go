package assembly_test

import (
	"testing"
)

func example() *string {
	myExample := "hello"
	return &myExample
}

func TestExample(t *testing.T) {
	actual := example()
	if *actual != "hello" {
		t.Fatal("test failure")
	}
}
