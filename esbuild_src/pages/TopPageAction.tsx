import React from "react";
import { createRoot } from "react-dom/client";

const App = () => {
    return <h2>CSR template(react)</h2>;
};
createRoot(document.body).render(<App />);
