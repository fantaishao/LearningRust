import * as $protobuf from "protobufjs";
import Long = require("long");
/** Namespace bookStorePackage. */
export namespace bookStorePackage {

    /** Represents a Book */
    class Book extends $protobuf.rpc.Service {

        /**
         * Constructs a new Book service.
         * @param rpcImpl RPC implementation
         * @param [requestDelimited=false] Whether requests are length-delimited
         * @param [responseDelimited=false] Whether responses are length-delimited
         */
        constructor(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean);

        /**
         * Creates new Book service using the specified rpc implementation.
         * @param rpcImpl RPC implementation
         * @param [requestDelimited=false] Whether requests are length-delimited
         * @param [responseDelimited=false] Whether responses are length-delimited
         * @returns RPC service. Useful where requests and/or responses are streamed.
         */
        public static create(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean): Book;

        /**
         * Calls createBook.
         * @param request BookItem message or plain object
         * @param callback Node-style callback called with the error, if any, and BookItem
         */
        public createBook(request: bookStorePackage.IBookItem, callback: bookStorePackage.Book.createBookCallback): void;

        /**
         * Calls createBook.
         * @param request BookItem message or plain object
         * @returns Promise
         */
        public createBook(request: bookStorePackage.IBookItem): Promise<bookStorePackage.BookItem>;

        /**
         * Calls readBook.
         * @param request BookRequest message or plain object
         * @param callback Node-style callback called with the error, if any, and BookItem
         */
        public readBook(request: bookStorePackage.IBookRequest, callback: bookStorePackage.Book.readBookCallback): void;

        /**
         * Calls readBook.
         * @param request BookRequest message or plain object
         * @returns Promise
         */
        public readBook(request: bookStorePackage.IBookRequest): Promise<bookStorePackage.BookItem>;

        /**
         * Calls readBooks.
         * @param request Empty message or plain object
         * @param callback Node-style callback called with the error, if any, and BooksList
         */
        public readBooks(request: bookStorePackage.IEmpty, callback: bookStorePackage.Book.readBooksCallback): void;

        /**
         * Calls readBooks.
         * @param request Empty message or plain object
         * @returns Promise
         */
        public readBooks(request: bookStorePackage.IEmpty): Promise<bookStorePackage.BooksList>;
    }

    namespace Book {

        /**
         * Callback as used by {@link bookStorePackage.Book#createBook}.
         * @param error Error, if any
         * @param [response] BookItem
         */
        type createBookCallback = (error: (Error|null), response?: bookStorePackage.BookItem) => void;

        /**
         * Callback as used by {@link bookStorePackage.Book#readBook}.
         * @param error Error, if any
         * @param [response] BookItem
         */
        type readBookCallback = (error: (Error|null), response?: bookStorePackage.BookItem) => void;

        /**
         * Callback as used by {@link bookStorePackage.Book#readBooks}.
         * @param error Error, if any
         * @param [response] BooksList
         */
        type readBooksCallback = (error: (Error|null), response?: bookStorePackage.BooksList) => void;
    }

    /** Properties of a BookItem. */
    interface IBookItem {

        /** BookItem id */
        id?: (number|null);

        /** BookItem book */
        book?: (string|null);
    }

    /** Represents a BookItem. */
    class BookItem implements IBookItem {

        /**
         * Constructs a new BookItem.
         * @param [properties] Properties to set
         */
        constructor(properties?: bookStorePackage.IBookItem);

        /** BookItem id. */
        public id: number;

        /** BookItem book. */
        public book: string;

        /**
         * Creates a new BookItem instance using the specified properties.
         * @param [properties] Properties to set
         * @returns BookItem instance
         */
        public static create(properties?: bookStorePackage.IBookItem): bookStorePackage.BookItem;

        /**
         * Encodes the specified BookItem message. Does not implicitly {@link bookStorePackage.BookItem.verify|verify} messages.
         * @param message BookItem message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: bookStorePackage.IBookItem, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified BookItem message, length delimited. Does not implicitly {@link bookStorePackage.BookItem.verify|verify} messages.
         * @param message BookItem message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: bookStorePackage.IBookItem, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a BookItem message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns BookItem
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): bookStorePackage.BookItem;

        /**
         * Decodes a BookItem message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns BookItem
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): bookStorePackage.BookItem;

        /**
         * Verifies a BookItem message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a BookItem message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns BookItem
         */
        public static fromObject(object: { [k: string]: any }): bookStorePackage.BookItem;

        /**
         * Creates a plain object from a BookItem message. Also converts values to other types if specified.
         * @param message BookItem
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: bookStorePackage.BookItem, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this BookItem to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };

        /**
         * Gets the default type url for BookItem
         * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
         * @returns The default type url
         */
        public static getTypeUrl(typeUrlPrefix?: string): string;
    }

    /** Properties of a BookRequest. */
    interface IBookRequest {

        /** BookRequest id */
        id?: (number|null);
    }

    /** Represents a BookRequest. */
    class BookRequest implements IBookRequest {

        /**
         * Constructs a new BookRequest.
         * @param [properties] Properties to set
         */
        constructor(properties?: bookStorePackage.IBookRequest);

        /** BookRequest id. */
        public id: number;

        /**
         * Creates a new BookRequest instance using the specified properties.
         * @param [properties] Properties to set
         * @returns BookRequest instance
         */
        public static create(properties?: bookStorePackage.IBookRequest): bookStorePackage.BookRequest;

        /**
         * Encodes the specified BookRequest message. Does not implicitly {@link bookStorePackage.BookRequest.verify|verify} messages.
         * @param message BookRequest message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: bookStorePackage.IBookRequest, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified BookRequest message, length delimited. Does not implicitly {@link bookStorePackage.BookRequest.verify|verify} messages.
         * @param message BookRequest message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: bookStorePackage.IBookRequest, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a BookRequest message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns BookRequest
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): bookStorePackage.BookRequest;

        /**
         * Decodes a BookRequest message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns BookRequest
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): bookStorePackage.BookRequest;

        /**
         * Verifies a BookRequest message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a BookRequest message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns BookRequest
         */
        public static fromObject(object: { [k: string]: any }): bookStorePackage.BookRequest;

        /**
         * Creates a plain object from a BookRequest message. Also converts values to other types if specified.
         * @param message BookRequest
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: bookStorePackage.BookRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this BookRequest to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };

        /**
         * Gets the default type url for BookRequest
         * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
         * @returns The default type url
         */
        public static getTypeUrl(typeUrlPrefix?: string): string;
    }

    /** Properties of a BooksList. */
    interface IBooksList {

        /** BooksList books */
        books?: (bookStorePackage.IBookItem[]|null);
    }

    /** Represents a BooksList. */
    class BooksList implements IBooksList {

        /**
         * Constructs a new BooksList.
         * @param [properties] Properties to set
         */
        constructor(properties?: bookStorePackage.IBooksList);

        /** BooksList books. */
        public books: bookStorePackage.IBookItem[];

        /**
         * Creates a new BooksList instance using the specified properties.
         * @param [properties] Properties to set
         * @returns BooksList instance
         */
        public static create(properties?: bookStorePackage.IBooksList): bookStorePackage.BooksList;

        /**
         * Encodes the specified BooksList message. Does not implicitly {@link bookStorePackage.BooksList.verify|verify} messages.
         * @param message BooksList message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: bookStorePackage.IBooksList, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified BooksList message, length delimited. Does not implicitly {@link bookStorePackage.BooksList.verify|verify} messages.
         * @param message BooksList message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: bookStorePackage.IBooksList, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a BooksList message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns BooksList
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): bookStorePackage.BooksList;

        /**
         * Decodes a BooksList message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns BooksList
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): bookStorePackage.BooksList;

        /**
         * Verifies a BooksList message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates a BooksList message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns BooksList
         */
        public static fromObject(object: { [k: string]: any }): bookStorePackage.BooksList;

        /**
         * Creates a plain object from a BooksList message. Also converts values to other types if specified.
         * @param message BooksList
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: bookStorePackage.BooksList, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this BooksList to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };

        /**
         * Gets the default type url for BooksList
         * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
         * @returns The default type url
         */
        public static getTypeUrl(typeUrlPrefix?: string): string;
    }

    /** Properties of an Empty. */
    interface IEmpty {
    }

    /** Represents an Empty. */
    class Empty implements IEmpty {

        /**
         * Constructs a new Empty.
         * @param [properties] Properties to set
         */
        constructor(properties?: bookStorePackage.IEmpty);

        /**
         * Creates a new Empty instance using the specified properties.
         * @param [properties] Properties to set
         * @returns Empty instance
         */
        public static create(properties?: bookStorePackage.IEmpty): bookStorePackage.Empty;

        /**
         * Encodes the specified Empty message. Does not implicitly {@link bookStorePackage.Empty.verify|verify} messages.
         * @param message Empty message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: bookStorePackage.IEmpty, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified Empty message, length delimited. Does not implicitly {@link bookStorePackage.Empty.verify|verify} messages.
         * @param message Empty message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: bookStorePackage.IEmpty, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes an Empty message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns Empty
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): bookStorePackage.Empty;

        /**
         * Decodes an Empty message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns Empty
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): bookStorePackage.Empty;

        /**
         * Verifies an Empty message.
         * @param message Plain object to verify
         * @returns `null` if valid, otherwise the reason why it is not
         */
        public static verify(message: { [k: string]: any }): (string|null);

        /**
         * Creates an Empty message from a plain object. Also converts values to their respective internal types.
         * @param object Plain object
         * @returns Empty
         */
        public static fromObject(object: { [k: string]: any }): bookStorePackage.Empty;

        /**
         * Creates a plain object from an Empty message. Also converts values to other types if specified.
         * @param message Empty
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: bookStorePackage.Empty, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this Empty to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };

        /**
         * Gets the default type url for Empty
         * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
         * @returns The default type url
         */
        public static getTypeUrl(typeUrlPrefix?: string): string;
    }
}
