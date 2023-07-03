import { data } from "../../data/data";

// Initial state for the reducer
export const defaultState = {
  people: data,
  isLoading: false, // Indicates if data is being loaded
};
