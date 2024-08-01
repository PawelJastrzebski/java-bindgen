import React from "react"
import "./Panel.scss"

export default function Panel({children, id, ...props}: {children?: React.ReactNode, id?: string} & Object) {
    return (
        <div {...props} id={id} className="app-panel">
            {children}
        </div>
    )
}