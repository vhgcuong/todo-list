export function getDateByFormat( date: Date, format: string = "Ymd"): string {
    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, "0")
    const day = String(date.getDate()).padStart(2, "0")
    const hour = String(date.getHours()).padStart(2, "0")
    const minute = String(date.getMinutes()).padStart(2, "0")
    const second = String(date.getSeconds()).padStart(2, "0")

    switch (format) {
        case "Ymd":
            return `${year}${month}${day}`
        case "Y-m-d":
            return `${year}-${month}-${day}`
        case "Y/m/d":
            return `${year}/${month}/${day}`
        case "Y-m-d H:i:s":
            return `${year}-${month}-${day} ${hour}:${minute}:${second}`
        default:
            return `${year}${month}${day}`
    }
}