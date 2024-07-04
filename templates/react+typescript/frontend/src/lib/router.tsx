import { createRoutesFromElements, createBrowserRouter, Route } from "react-router-dom";
import HomePage from "../pages/HomePage";

export const router = createBrowserRouter(
    createRoutesFromElements(
        <>
            <Route path="/" element={< HomePage />} />
        </>
    )
);
