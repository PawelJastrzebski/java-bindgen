


export const processFile = async (image: File, transforms: string[]) => {
    let ext: string = image.name.split('.').pop() ?? "jpg"

    const formData = new FormData();
    formData.append("image", image)
    formData.append("imageExtension", ext)
    formData.append("transforms", transforms.join(","))


    const response = await fetch("http://localhost:6060/img/process", {
        method: 'POST',
        body: formData,
        headers: {
            // "Accept": "image/*"
        }
    });

    console.log(response)
}