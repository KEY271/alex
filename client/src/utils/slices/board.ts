import { createSlice, PayloadAction } from "@reduxjs/toolkit";

interface BoardState {
    history: number[];
}

const boardSlice = createSlice({
    name: "board",
    initialState: { history: [] } as BoardState,
    reducers: {
        setHistory: (state, action: PayloadAction<number[]>) => {
            state.history = action.payload;
        }
    }
});

export const { setHistory } = boardSlice.actions;
export default boardSlice.reducer;
