import { MessageContent } from "../../model/messages";
import { fromCandid as optionFromCandid, toCandid as optionToCandid } from "../candidConverters/option";

export function fromCandid(content: any) : MessageContent {
    if (content.hasOwnProperty("Text")) {
        const inner = content.Text;
        return {
            kind: "text",
            text: inner.text
        };
    }
    if (content.hasOwnProperty("Media")) {
        const inner = content.Media;
        return {
            kind: "media",
            caption: optionFromCandid(inner.caption),
            mimeType: inner.mime_type,
            id: inner.blob_id,
            size: inner.blob_size,
            chunkSize: inner.chunk_size
        };
    }
    if (content.hasOwnProperty("File")) {
        const inner = content.File;
        return {
            kind: "file",
            name: inner.name,
            mimeType: inner.mime_type,
            id: inner.blob_id,
            size: inner.blob_size,
            chunkSize: inner.chunk_size
        };
    }
    throw new Error("Unrecognised content type - " + JSON.stringify(content));
}

export function toCandid(content: MessageContent) : any {

    switch (content.kind) {
        case "text":
            return {
                Text: {
                    text: content.text
                }
            };
        case "media":
            return {
                Media: {
                    caption: optionToCandid(content.caption),
                    mime_type: content.mimeType,
                    blob_id: content.id,
                    blob_size: content.size,
                    chunk_size: content.chunkSize
                }
            };
        case "file":
            return {
                File: {
                    name: content.name,
                    mime_type: content.mimeType,
                    blob_id: content.id,
                    blob_size: content.size,
                    chunk_size: content.chunkSize
                }
            };
    }
}
