"use strict";
function add(n1, n2, showResult, phrase) {
    const result = n1 + n2;
    if (showResult) {
        alert(phrase + result);
    }
    else {
        return result;
    }
}
function combine(n1, n2, showResult) {
    let result;
    if (typeof (n1 === "number" && n2 === "number")) {
        return result;
    }
    else {
        return n1.toString() + n2.toString();
    }
}
const n1 = 5;
const n2 = 2.8;
const printResult = true;
const resultPhrase = "Result is: ";
add(n1, n2, printResult, resultPhrase);
var Role;
(function (Role) {
    Role[Role["ADMIN"] = 0] = "ADMIN";
    Role[Role["READ_ONLY"] = 1] = "READ_ONLY";
    Role[Role["AUTHOR"] = 2] = "AUTHOR";
})(Role || (Role = {}));
const person = {
    name: "Max",
    age: 40,
    role: Role.ADMIN,
};
alert(person.age + " " + person.name);
function subtract(n1, n2) {
    return n1 - n2;
}
let userInput;
const button = document.querySelector("button");
button?.addEventListener("click", () => {
    console.log("Clicked!");
});
//# sourceMappingURL=app.js.map