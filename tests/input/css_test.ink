// Test CSS styling on document-level elements

document {
    text (id="header", class="intro") {
        "Welcome to the document"
    }
    text (class="bodytext") {
        "This is the body content"
    }
    section (id="mainsection") {
        text (class="nested") {
            "Nested text in section"
        }
    }
}

style {
    #header {
        font-size = 24
        color = "blue"
    }

    .intro {
        font-weight = "bold"
    }

    .bodytext {
        font-size = 12
        color = "black"
    }

    section {
        margin = 20
        padding = 10
    }

    .nested {
        font-style = "italic"
    }
}
