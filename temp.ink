// Sample PDF markdown test file

template {
    title = "My Document"       // this is a default variable for the document
    author = "Alice"            // this is a default variable for the document
    font_size = 12              // this is a default variable for the document
    
    // Simple formula
    let total_price = $price * quantity$ // this is not a default value and needs to be defined with "let" or "const"

    func intro_section() {
        return "introduction, the total price is {total_price}"
    }
}

document {
    intro_section(ClassName=intro) // section has default attributes that can be called
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
