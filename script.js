document.addEventListener("DOMContentLoaded", () => {
    const form = document.getElementById("bodmasform");
    const input1 = document.getElementById("num1");
    const input2 = document.getElementById("num2");
    const resultoutput = document.getElementById("result");

    const operations = {
        add: (a, b) => a + b,
        sub: (a, b) => a - b,
        mul: (a, b) => a * b,
        div: (a, b) => {
            if (b === 0 ) throw new Error("Division by zero is not allowed.")
                return a / b;
        }
    };

    form.addEventListener("submit", (event) => {
        event.preventDefault();

        const a = parseFloat(input1.value) || 0;
        const b = parseFloat(input2.value) || 0;

        const selected = document.querySelector('input[name="opt-name"]:checked');
        if (!selected) {
            showerror("Please select an operator.");
            return;
        }

        try {
            const operation = operations[selected.value];
            const result = operation(a, b);
            resultoutput.value = result;
        } catch (error) {
            showerror(error.message);
        }
    });

    document.querySelectorAll("[data-reset]").forEach(button => {
        button.addEventListener("click", () => {
            const target = document.getElementById(button.dataset.reset);
            target.value = "";
            target.focus();
        });
    });

    function showerror(message) {
        alert(message);
    }
});