// Sample PDF markdown test file

template {
    title = "My Document"       // this is a default variable for the document
    author = "Alice"            // this is a default variable for the document
    font_size = 12              // this is a default variable for the document

    // Simple formula
    let total_price = "$price * quantity$" // this is not a default value and needs to be defined with "let" or "const"

    const tax_rate = 0.08       // this is a constant value that can be used throughout the document

    func intro_section(param1: String, param2: Int) {
        return "introduction, the total price is {total_price}"
    }
}

document {
    let number = 42
    intro_section("name", number, class="intro") // section has default attributes that can be called
    text {
        this is also text that can be parsed by the compiler
    }
    list {
        - this is a list
        - defined by its dashes (and the name "list")
    }
}

style {
    body {
        font-family = "Helvetica"
        color = "black"
        margin = 1.0
    }

    .intro {
        font-size = 24          // overloaded font size
        font-weight = "bold"    // overloaded entire section styling
    }
}

/* End of test file */
