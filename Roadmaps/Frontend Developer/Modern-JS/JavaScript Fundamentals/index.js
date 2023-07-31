"use strict";

var paragraph = document.getElementById("p");
var text = document.createTextNode(" .This just got added.");

paragraph.appendChild(text);

alert("Hello");

[1, 2].forEach(alert);

const bigInt = 1234567890123456789012345678901234567890n;
alert(bigInt);

function myFunction() {
  let person = prompt("Please enter your name", "Harry Potter");
  if (person != null) {
    document.getElementById("demo").innerHTML =
      "Hello " + person + "! How are you today?";
  }
}

let isBoss = confirm("Are you the boss?");
alert(isBoss); // true if OK is pressed

alert(5 > 4);
alert("apple" > "pineapple");
alert("2" > "12");
alert(undefined == null);
alert(undefined === null);
alert(null == "\n0\n");
alert(null === +"\n0\n");

let user;
alert(user ?? "Anonymous");

let first = null;
let last = undefined;
let middle = "middle";
alert(first ?? last ?? middle ?? "Anonymous");
alert(first || last || middle || "anonymous");
alert((1 && 2) ?? 3);
