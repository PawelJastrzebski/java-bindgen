import { Button, Input, Link, Tab, Tabs } from "@nextui-org/react"
import "./LoginPage.scss"
import { useState } from "react";
import { EyeFilledIcon } from "../ui/icons/EyeFilledIcon"
import { EyeSlashIcon } from "../ui/icons/EyeSlashIcon"


function LoginForm() {
    const [isVisible, setIsVisible] = useState(false);
    const toggleVisibility = () => setIsVisible(!isVisible);

    return (
        <div className="dark box-border flex flex-col py-4 md:flex-nowrap gap-4">
            <Input
                isRequired
                size="sm"
                required={true}
                type="email"
                label="Email " />
            <Input
                isRequired
                size="sm"
                required={true}
                label="Password "
                type={isVisible ? "text" : "password"}
                endContent={
                    <button className="focus:outline-none" type="button" onClick={toggleVisibility} aria-label="toggle password visibility">
                        {isVisible ? (
                            <EyeSlashIcon className="text-2xl text-default-400 pointer-events-none" />
                        ) : (
                            <EyeFilledIcon className="text-2xl text-default-400 pointer-events-none" />
                        )}
                    </button>
                }
            />
            <Button color="primary" size="md" variant="solid">
                Login
            </Button>
            <p className="text-center text-small text-zinc-300">
                Need to create an account?{" "}
                <Link className="cursor-pointer" size="sm" onPress={() => console.log("sign-up")}>
                    Sign up
                </Link>
            </p>

        </div>
    )
}

export default function LoginPage() {

    return (
        <div id="login-page" className="w-full h-full">


            <div className="left-bar p-3 bg-zinc-900">
                <h2 className="app-logo">Java Bindgen</h2>
                <Tabs fullWidth color={"primary"} aria-label="Tabs colors" className="dark" size="md" radius="md">
                    <Tab key="login" title="Login" />
                    <Tab key="register" title="Register" />
                </Tabs>
                <LoginForm />
            </div>

            <div className="w-full app-body">
                <h2 className="text-zinc-900 text-xl text-left py-4 font-bold font-mono">Image Processing</h2>
            </div>


        </div>
    )
}