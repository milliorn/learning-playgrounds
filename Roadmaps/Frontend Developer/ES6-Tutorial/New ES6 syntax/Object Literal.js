function createMachine(name, status) {
  return {
    name: name,
    status: status,
  };
}

function createMachines(name, status) {
  return {
    name,
    status,
  };
}

let names = "machine name";

let machine = {
  [names]: "server",
  "machine hours": 10000,
};

console.log(machine[names]); // server
console.log(machine["machine hours"]); // 10000

let prefix = "machine";

let machines = {
  [prefix + " name"]: "server",
  [prefix + " hours"]: 10000,
};

console.log(machines["machine name"]); // server
console.log(machines["machine hours"]); // 10000

let server = {
  name: "Server",
  restart: function () {
    console.log("The" + this.name + " is restarting...");
  },
};

server = {
  name: "Server",
  restart() {
    console.log("The" + this.name + " is restarting...");
  },
};

server = {
  name: "Server",
  restart() {
    console.log("The " + this.name + " is restarting...");
  },
  "starting up"() {
    console.log("The " + this.name + " is starting up!");
  },
};

server["starting up"]();
