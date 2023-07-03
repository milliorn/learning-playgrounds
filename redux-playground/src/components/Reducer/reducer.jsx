import { data } from "../../data/data";

// Reducer function to handle state updates
export const reducer = (state, action) => {
  switch (action.type) {
    case "CLEAR_LIST":
      // Clear the list of people
      return { ...state, people: [] };
    case "REMOVE_ITEM":
      // Remove an item/person from the list based on the provided ID
      const newPeople = state.people.filter(
        (person) => person.id !== action.payload
      );
      return { ...state, people: newPeople };
    case "RESET_LIST":
      // Reset the list to its initial state
      return { ...state, people: data };
    default:
      throw new Error(`No matching action type: ${action.type}`); // Throw an error if an unknown action type is encountered
  }
};
