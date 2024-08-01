import "./ImageDropzone.scss"
import { useCallback, useEffect, useState } from 'react'
import { useDropzone } from 'react-dropzone'

export default function ImageDropZone({ onSelect }: { onSelect: (file: File | null) => void }) {
    const [img, setImg] = useState(null as File | null)
    const [imgUrl, setImgUrl] = useState(null as string | null)

    useEffect(() => {
        if (!img) {
            setImgUrl(null);
            onSelect(img)
            return;
        }
        var reader = new FileReader();
        reader.onload = () => {
            setImgUrl(reader.result as string)
            onSelect(img)
        };
        reader.readAsDataURL(img as File);
    }, [img])

    const onDrop = useCallback((acceptedFiles: File[]) => {
        if (acceptedFiles[0]) {
            setImg(acceptedFiles[0])
        }
    }, [])
    const { getRootProps, getInputProps, isDragActive } = useDropzone({ onDrop })


    let hint = img == null ? <p>Drag 'n' drop some images here, or click to select files</p> : <></>
    return (
        <div id="image-drop-zone" {...getRootProps()}>
            <input {...getInputProps()} />
            <div className="overlay">
                {
                    isDragActive ?
                        <p className="drop-hint">Drop image files here ...</p> :
                        hint
                }
            </div>
            <img src={imgUrl ?? ""} />

        </div>
    )
}