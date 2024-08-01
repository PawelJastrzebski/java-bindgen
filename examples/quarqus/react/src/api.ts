
export const processFile = async (image: File, transforms: string[]) => {
    let ext: string = image.name.split('.').pop() ?? "jpg"

    const formData = new FormData();
    formData.append("image", image)
    formData.append("imageExtension", ext)
    formData.append("transforms", transforms.join(";"))

    const res = await fetch("http://localhost:6060/img/process", {
        method: 'POST',
        body: formData,
        headers: {}
    });
    const blob = await res.blob();
    console.log({res, input_transforms: transforms})
    return blob
}