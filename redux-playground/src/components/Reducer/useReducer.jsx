import React, { useReducer } from "react";
import { defaultState } from "./defaultState";
import { reducer } from "./reducer";

const ReducerBasics = () => {
  const [state, dispatch] = useReducer(reducer, defaultState);

  // Function to remove an item/person from the list
  const removeItem = (id) => {
    dispatch({ type: "REMOVE_ITEM", payload: id });
  };

  // Function to clear the entire list of people
  const clearList = () => {
    dispatch({ type: "CLEAR_LIST" });
  };

  // Function to reset the list to its initial state
  const resetList = () => {
    dispatch({ type: "RESET_LIST" });
  };

  console.log(state); // Log the current state to the console

  return (
    <div>
      {/* Render each person as a separate item */}
      {state.people.map((person) => {
        const { id, name } = person;
        return (
          <div key={id} className="item">
            <h4>{name}</h4>
            <button onClick={() => removeItem(id)}>remove</button>
          </div>
        );
      })}
      {/* Render different buttons based on the length of the people list */}
      {state.people.length < 1 ? (
        <button
          className="btn"
          style={{ marginTop: "2rem" }}
          onClick={resetList}
        >
          reset
        </button>
      ) : (
        <button
          className="btn"
          style={{ marginTop: "2rem" }}
          onClick={clearList}
        >
          clear
        </button>
      )}
    </div>
  );
};

export default ReducerBasics;
