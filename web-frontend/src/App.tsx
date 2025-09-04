import { onAuthStateChanged } from 'firebase/auth';
import { auth } from './firebase';
import {useState} from "react";
import Form from "./components/Form.tsx";
import Login from "./components/Login.tsx";

function App() {

  const [loggedIn, setLoggedIn] = useState(false);

  onAuthStateChanged(auth, (user) => {
    if (user) {
      setLoggedIn(true);
    } else {
      setLoggedIn(false);
    }
  });

  return (
    <>
      {loggedIn ? <Form/> : <Login/>}
    </>
  )
}

export default App
