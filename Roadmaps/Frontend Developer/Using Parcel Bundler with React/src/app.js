import { ReactDOM } from "react";
import React from "react";

export const app = ({...rest}) => {
  return (
    <div>
      <h1>Hello World</h1>
    </div>
  );
};

ReactDOM.render(<App />, document.getElementById("root"));
