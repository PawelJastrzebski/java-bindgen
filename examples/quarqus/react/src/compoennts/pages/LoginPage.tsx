import { Button, Input, Link, Slider, Tab, Tabs } from "@nextui-org/react"
import "./LoginPage.scss"
import { useEffect, useState } from "react";
import { EyeFilledIcon } from "../ui/icons/EyeFilledIcon"
import { EyeSlashIcon } from "../ui/icons/EyeSlashIcon"
import ImageDropZone, { readBlob, SELECTED_FILE } from "../ui/ImageDropZone";
import { processFile } from "../../api";
import Panel from "../ui/Panel";

const NONE_VALUE = -10_000;

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


export function ImageResize({ onChange }: { onChange: (transform: string | null) => any }) {
    const [W, setW] = useState(NONE_VALUE)
    const [H, setH] = useState(NONE_VALUE)
    useEffect(() => {
        if (W == NONE_VALUE || H == NONE_VALUE) {
            onChange(null)
        } else {
            onChange(`resize:${W},${H}`)
        }
    }, [W, H])

    return (
        <Panel>
            <h2>Size</h2>
            <Slider
                size="sm"
                color="primary"
                label="Width"
                showSteps={true}
                maxValue={1920}
                minValue={1}
                defaultValue={400}
                getValue={(v) => `${v} px`}
                onChange={(v) => {
                    if (typeof v == "number") {
                        setW(v)
                    }
                }}
                className="max-w-md"
            />
            <Slider
                size="sm"
                color="primary"
                label="Height"
                showSteps={true}
                maxValue={1080}
                minValue={1}
                defaultValue={400}
                getValue={(v) => `${v} px`}
                onChange={(v) => {
                    if (typeof v == "number") {
                        setH(v)
                    }
                }}
                className="max-w-md"
            />
        </Panel>
    )
}

export function ImageContrast({ onChange }: { onChange: (transform: string | null) => any }) {
    const [V, setV] = useState(0)
    useEffect(() => {
        if (V == 0) {
            onChange(null)
        } else {
            onChange(`contrast:${V}`)
        }
    }, [V])

    return (
        <Panel>
            <h2>Contrast</h2>
            <Slider
                size="sm"
                color="primary"
                label="Value"
                step={0.1}
                showSteps={true}
                maxValue={10}
                minValue={-10}
                defaultValue={0}
                getValue={(v) => `${v}`}
                onChange={(v) => {
                    if (typeof v == "number") {
                        setV(v)
                    }
                }}
                className="max-w-md"
            />
        </Panel>
    )
}

let transforms: string[] = []

export function ImageProcessing() {
    const [output, setOptput] = useState("")
    const onSelect = async (file: File | null) => {
        if (!file) return;
        setOptput("")
        let result = await processFile(file, transforms)
        let url = await readBlob(result)
        setOptput(url)
    }


    const onChange = async (prefix: string, def: string | null) => {
        transforms = transforms.filter(t => !t.startsWith(prefix))
        if (!!def) {
            transforms.push(def)
        }

        const file = SELECTED_FILE;
        if (!file) return;
        let result = await processFile(file, transforms)
        let url = await readBlob(result)
        setOptput(url)
    }

    return (
        <>
            <h2 className="text-zinc-900 text-xl text-left py-4 font-bold font-mono">Image Processing</h2>
            <Tabs disabledKeys={output == "" ? ["output"] : []} aria-label="Options" color="primary" >
                <Tab className="tab" key="input" title="Input">
                    <ImageDropZone onSelect={onSelect} />
                </Tab>
                <Tab className="tab" key="output" title="Optput">
                    <Panel>
                        <img src={output} />
                    </Panel>
                </Tab>
            </Tabs>

            <div id="image-transforms" className="flex">
                <ImageResize onChange={(def) => onChange("resize:", def)} />
                <ImageContrast onChange={(def) => onChange("contrast:", def)} />
            </div>
        </>
    )
}

export default function LoginPage() {

    return (
        <div id="login-page" className="w-full h-full">

            <div className="left-bar p-3">
                <h2 className="app-logo">Java Bindgen</h2>
                <Tabs fullWidth color={"primary"} aria-label="Tabs colors" className="dark" size="md" radius="md">
                    <Tab key="login" title="Login" />
                    <Tab key="register" title="Register" />
                </Tabs>
                <LoginForm />
            </div>

            <div className="w-full app-body">
                <ImageProcessing />
            </div>

        </div>
    )
}