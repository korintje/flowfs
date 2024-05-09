import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";
import Dashboard from "./Dashboard";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Dashboard />
  </React.StrictMode>,
);
