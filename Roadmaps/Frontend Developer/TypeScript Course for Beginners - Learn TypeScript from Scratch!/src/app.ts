type Combo = number;

function add(n1: number, n2: number, showResult: boolean, phrase: string) {
  const result = n1 + n2;
  if (showResult) {
    alert(phrase + result);
  } else {
    return result;
  }
}

function combine(
  n1: number | string,
  n2: number | string,
  showResult: boolean
) {
  let result;
  if (typeof (n1 === "number" && n2 === "number")) {
    return result;
  } else {
    return n1.toString() + n2.toString();
  }
}

const n1 = 5;
const n2 = 2.8;
const printResult = true;
const resultPhrase = "Result is: ";

add(n1, n2, printResult, resultPhrase);

enum Role {
  ADMIN,
  READ_ONLY,
  AUTHOR,
}

const person: {
  name: string;
  age: number;
  role: Role;
} = {
  name: "Max",
  age: 40,
  role: Role.ADMIN,
};

alert(person.age + " " + person.name);

function subtract(n1: number, n2: number): number {
  return n1 - n2;
}

let userInput: unknown;

const button = document.querySelector("button");

button?.addEventListener("click", () => {
  console.log("Clicked!");
});
