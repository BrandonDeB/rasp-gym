import {Button} from "@mui/material";
import {useState} from "react";
import {GithubAuthProvider, signInWithPopup} from "firebase/auth";
import {auth} from "../firebase.tsx";

function Login() {

    const [isPending, setIsPending] = useState(false);
    const provider = new GithubAuthProvider();

    const login = async () => {
        setIsPending(true);

        const res = await signInWithPopup(auth, provider);
        if (!res) {
            console.log("Error logging in");
            setIsPending(false);
        } else {
            const user = res.user;
            console.log(user);
            setIsPending(false)
        }
    };

    return (
        <Button variant="outlined" onClick={login}>{isPending ? "Pending" : "Login with Github"}</Button>
    );
}

export default Login;