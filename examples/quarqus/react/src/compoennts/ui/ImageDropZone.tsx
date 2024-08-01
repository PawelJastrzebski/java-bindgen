import "./ImageDropzone.scss"
import { useCallback, useEffect, useState } from 'react'
import { useDropzone } from 'react-dropzone'
import Panel from "./Panel"

export const readBlob: (blob: File | Blob) => Promise<string> = (blob) => {
    return new Promise<string>((res, rej) => {
        var reader = new FileReader();
        reader.onload = () => {
            res(reader.result as string)
        };
        reader.onabort = rej
        reader.onerror = rej
        reader.readAsDataURL(blob);
    })
}

export let SELECTED_FILE: File | null = null;
export let SELECTED_FILE_URL: string | null = null;

export default function ImageDropZone({ onSelect }: { onSelect: (file: File | null) => void }) {
    const [img, setImg] = useState(SELECTED_FILE)
    const [imgUrl, setImgUrl] = useState(SELECTED_FILE_URL)

    useEffect(() => {
        if (!img) {
            setImgUrl(null);
            onSelect(null)
            return;
        }

        if (img == SELECTED_FILE) {
            return;
        }

        readBlob(img).then((url) => {
            setImgUrl(url)
            onSelect(img)
            SELECTED_FILE = img;
            SELECTED_FILE_URL = url
        })

    }, [img])

    const onDrop = useCallback((acceptedFiles: File[]) => {
        if (acceptedFiles[0]) {
            setImg(acceptedFiles[0])
        }
    }, [])
    const { getRootProps, getInputProps, isDragActive } = useDropzone({ onDrop })


    let hint = img == null ? <p>Drag 'n' drop some images here, or click to select files</p> : <></>
    return (

        <Panel id="image-drop-zone" {...getRootProps()}>
            <input {...getInputProps()} />
            <div className="overlay">
                {
                    isDragActive ?
                        <p className="drop-hint">Drop image files here ...</p> :
                        hint
                }
            </div>
            <img src={imgUrl ?? ""} />
        </Panel>

    )
}