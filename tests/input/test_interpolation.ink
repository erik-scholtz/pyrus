// Test string interpolation in HLIR

template {
    let name = "World"
    let greeting = "Hello, {name}!"
    let number = 42
    let message = "The number is {number}"

    func greet() {
        return text {
            "{greeting} - Value: {number}"
        }
    }
}

document {
    greet()
}

style {
    body {
        font-family = "Helvetica"
    }
}
