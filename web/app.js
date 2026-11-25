const add = (a, b) => a + b;
const double = (a) => a * 2;

const form = document.getElementById("calculator");
const inputA = document.getElementById("a");
const inputB = document.getElementById("b");
const result = document.getElementById("result");

function parseValue(input) {
  const value = Number.parseInt(input.value, 10);
  return Number.isNaN(value) ? 0 : value;
}

function render(value) {
  result.textContent = `Result: ${value}`;
}

form.addEventListener("submit", (event) => {
  event.preventDefault();

  const op = event.submitter?.value ?? "add";
  const a = parseValue(inputA);
  const b = parseValue(inputB);

  render(op === "double" ? double(a) : add(a, b));
});
