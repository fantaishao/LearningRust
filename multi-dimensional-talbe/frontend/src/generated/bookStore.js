/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
import * as $protobuf from "protobufjs/minimal";

// Common aliases
const $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;

// Exported root namespace
const $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});

export const bookStorePackage = $root.bookStorePackage = (() => {

    /**
     * Namespace bookStorePackage.
     * @exports bookStorePackage
     * @namespace
     */
    const bookStorePackage = {};

    bookStorePackage.Book = (function() {

        /**
         * Constructs a new Book service.
         * @memberof bookStorePackage
         * @classdesc Represents a Book
         * @extends $protobuf.rpc.Service
         * @constructor
         * @param {$protobuf.RPCImpl} rpcImpl RPC implementation
         * @param {boolean} [requestDelimited=false] Whether requests are length-delimited
         * @param {boolean} [responseDelimited=false] Whether responses are length-delimited
         */
        function Book(rpcImpl, requestDelimited, responseDelimited) {
            $protobuf.rpc.Service.call(this, rpcImpl, requestDelimited, responseDelimited);
        }

        (Book.prototype = Object.create($protobuf.rpc.Service.prototype)).constructor = Book;

        /**
         * Creates new Book service using the specified rpc implementation.
         * @function create
         * @memberof bookStorePackage.Book
         * @static
         * @param {$protobuf.RPCImpl} rpcImpl RPC implementation
         * @param {boolean} [requestDelimited=false] Whether requests are length-delimited
         * @param {boolean} [responseDelimited=false] Whether responses are length-delimited
         * @returns {Book} RPC service. Useful where requests and/or responses are streamed.
         */
        Book.create = function create(rpcImpl, requestDelimited, responseDelimited) {
            return new this(rpcImpl, requestDelimited, responseDelimited);
        };

        /**
         * Callback as used by {@link bookStorePackage.Book#createBook}.
         * @memberof bookStorePackage.Book
         * @typedef createBookCallback
         * @type {function}
         * @param {Error|null} error Error, if any
         * @param {bookStorePackage.BookItem} [response] BookItem
         */

        /**
         * Calls createBook.
         * @function createBook
         * @memberof bookStorePackage.Book
         * @instance
         * @param {bookStorePackage.IBookItem} request BookItem message or plain object
         * @param {bookStorePackage.Book.createBookCallback} callback Node-style callback called with the error, if any, and BookItem
         * @returns {undefined}
         * @variation 1
         */
        Object.defineProperty(Book.prototype.createBook = function createBook(request, callback) {
            return this.rpcCall(createBook, $root.bookStorePackage.BookItem, $root.bookStorePackage.BookItem, request, callback);
        }, "name", { value: "createBook" });

        /**
         * Calls createBook.
         * @function createBook
         * @memberof bookStorePackage.Book
         * @instance
         * @param {bookStorePackage.IBookItem} request BookItem message or plain object
         * @returns {Promise<bookStorePackage.BookItem>} Promise
         * @variation 2
         */

        /**
         * Callback as used by {@link bookStorePackage.Book#readBook}.
         * @memberof bookStorePackage.Book
         * @typedef readBookCallback
         * @type {function}
         * @param {Error|null} error Error, if any
         * @param {bookStorePackage.BookItem} [response] BookItem
         */

        /**
         * Calls readBook.
         * @function readBook
         * @memberof bookStorePackage.Book
         * @instance
         * @param {bookStorePackage.IBookRequest} request BookRequest message or plain object
         * @param {bookStorePackage.Book.readBookCallback} callback Node-style callback called with the error, if any, and BookItem
         * @returns {undefined}
         * @variation 1
         */
        Object.defineProperty(Book.prototype.readBook = function readBook(request, callback) {
            return this.rpcCall(readBook, $root.bookStorePackage.BookRequest, $root.bookStorePackage.BookItem, request, callback);
        }, "name", { value: "readBook" });

        /**
         * Calls readBook.
         * @function readBook
         * @memberof bookStorePackage.Book
         * @instance
         * @param {bookStorePackage.IBookRequest} request BookRequest message or plain object
         * @returns {Promise<bookStorePackage.BookItem>} Promise
         * @variation 2
         */

        /**
         * Callback as used by {@link bookStorePackage.Book#readBooks}.
         * @memberof bookStorePackage.Book
         * @typedef readBooksCallback
         * @type {function}
         * @param {Error|null} error Error, if any
         * @param {bookStorePackage.BooksList} [response] BooksList
         */

        /**
         * Calls readBooks.
         * @function readBooks
         * @memberof bookStorePackage.Book
         * @instance
         * @param {bookStorePackage.IEmpty} request Empty message or plain object
         * @param {bookStorePackage.Book.readBooksCallback} callback Node-style callback called with the error, if any, and BooksList
         * @returns {undefined}
         * @variation 1
         */
        Object.defineProperty(Book.prototype.readBooks = function readBooks(request, callback) {
            return this.rpcCall(readBooks, $root.bookStorePackage.Empty, $root.bookStorePackage.BooksList, request, callback);
        }, "name", { value: "readBooks" });

        /**
         * Calls readBooks.
         * @function readBooks
         * @memberof bookStorePackage.Book
         * @instance
         * @param {bookStorePackage.IEmpty} request Empty message or plain object
         * @returns {Promise<bookStorePackage.BooksList>} Promise
         * @variation 2
         */

        return Book;
    })();

    bookStorePackage.BookItem = (function() {

        /**
         * Properties of a BookItem.
         * @memberof bookStorePackage
         * @interface IBookItem
         * @property {number|null} [id] BookItem id
         * @property {string|null} [book] BookItem book
         */

        /**
         * Constructs a new BookItem.
         * @memberof bookStorePackage
         * @classdesc Represents a BookItem.
         * @implements IBookItem
         * @constructor
         * @param {bookStorePackage.IBookItem=} [properties] Properties to set
         */
        function BookItem(properties) {
            if (properties)
                for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * BookItem id.
         * @member {number} id
         * @memberof bookStorePackage.BookItem
         * @instance
         */
        BookItem.prototype.id = 0;

        /**
         * BookItem book.
         * @member {string} book
         * @memberof bookStorePackage.BookItem
         * @instance
         */
        BookItem.prototype.book = "";

        /**
         * Creates a new BookItem instance using the specified properties.
         * @function create
         * @memberof bookStorePackage.BookItem
         * @static
         * @param {bookStorePackage.IBookItem=} [properties] Properties to set
         * @returns {bookStorePackage.BookItem} BookItem instance
         */
        BookItem.create = function create(properties) {
            return new BookItem(properties);
        };

        /**
         * Encodes the specified BookItem message. Does not implicitly {@link bookStorePackage.BookItem.verify|verify} messages.
         * @function encode
         * @memberof bookStorePackage.BookItem
         * @static
         * @param {bookStorePackage.IBookItem} message BookItem message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BookItem.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && Object.hasOwnProperty.call(message, "id"))
                writer.uint32(/* id 1, wireType 0 =*/8).int32(message.id);
            if (message.book != null && Object.hasOwnProperty.call(message, "book"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.book);
            return writer;
        };

        /**
         * Encodes the specified BookItem message, length delimited. Does not implicitly {@link bookStorePackage.BookItem.verify|verify} messages.
         * @function encodeDelimited
         * @memberof bookStorePackage.BookItem
         * @static
         * @param {bookStorePackage.IBookItem} message BookItem message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BookItem.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a BookItem message from the specified reader or buffer.
         * @function decode
         * @memberof bookStorePackage.BookItem
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {bookStorePackage.BookItem} BookItem
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BookItem.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            let end = length === undefined ? reader.len : reader.pos + length, message = new $root.bookStorePackage.BookItem();
            while (reader.pos < end) {
                let tag = reader.uint32();
                switch (tag >>> 3) {
                case 1: {
                        message.id = reader.int32();
                        break;
                    }
                case 2: {
                        message.book = reader.string();
                        break;
                    }
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a BookItem message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof bookStorePackage.BookItem
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {bookStorePackage.BookItem} BookItem
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BookItem.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a BookItem message.
         * @function verify
         * @memberof bookStorePackage.BookItem
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        BookItem.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isInteger(message.id))
                    return "id: integer expected";
            if (message.book != null && message.hasOwnProperty("book"))
                if (!$util.isString(message.book))
                    return "book: string expected";
            return null;
        };

        /**
         * Creates a BookItem message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof bookStorePackage.BookItem
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {bookStorePackage.BookItem} BookItem
         */
        BookItem.fromObject = function fromObject(object) {
            if (object instanceof $root.bookStorePackage.BookItem)
                return object;
            let message = new $root.bookStorePackage.BookItem();
            if (object.id != null)
                message.id = object.id | 0;
            if (object.book != null)
                message.book = String(object.book);
            return message;
        };

        /**
         * Creates a plain object from a BookItem message. Also converts values to other types if specified.
         * @function toObject
         * @memberof bookStorePackage.BookItem
         * @static
         * @param {bookStorePackage.BookItem} message BookItem
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        BookItem.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            let object = {};
            if (options.defaults) {
                object.id = 0;
                object.book = "";
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.book != null && message.hasOwnProperty("book"))
                object.book = message.book;
            return object;
        };

        /**
         * Converts this BookItem to JSON.
         * @function toJSON
         * @memberof bookStorePackage.BookItem
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        BookItem.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        /**
         * Gets the default type url for BookItem
         * @function getTypeUrl
         * @memberof bookStorePackage.BookItem
         * @static
         * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
         * @returns {string} The default type url
         */
        BookItem.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
            if (typeUrlPrefix === undefined) {
                typeUrlPrefix = "type.googleapis.com";
            }
            return typeUrlPrefix + "/bookStorePackage.BookItem";
        };

        return BookItem;
    })();

    bookStorePackage.BookRequest = (function() {

        /**
         * Properties of a BookRequest.
         * @memberof bookStorePackage
         * @interface IBookRequest
         * @property {number|null} [id] BookRequest id
         */

        /**
         * Constructs a new BookRequest.
         * @memberof bookStorePackage
         * @classdesc Represents a BookRequest.
         * @implements IBookRequest
         * @constructor
         * @param {bookStorePackage.IBookRequest=} [properties] Properties to set
         */
        function BookRequest(properties) {
            if (properties)
                for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * BookRequest id.
         * @member {number} id
         * @memberof bookStorePackage.BookRequest
         * @instance
         */
        BookRequest.prototype.id = 0;

        /**
         * Creates a new BookRequest instance using the specified properties.
         * @function create
         * @memberof bookStorePackage.BookRequest
         * @static
         * @param {bookStorePackage.IBookRequest=} [properties] Properties to set
         * @returns {bookStorePackage.BookRequest} BookRequest instance
         */
        BookRequest.create = function create(properties) {
            return new BookRequest(properties);
        };

        /**
         * Encodes the specified BookRequest message. Does not implicitly {@link bookStorePackage.BookRequest.verify|verify} messages.
         * @function encode
         * @memberof bookStorePackage.BookRequest
         * @static
         * @param {bookStorePackage.IBookRequest} message BookRequest message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BookRequest.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && Object.hasOwnProperty.call(message, "id"))
                writer.uint32(/* id 1, wireType 0 =*/8).int32(message.id);
            return writer;
        };

        /**
         * Encodes the specified BookRequest message, length delimited. Does not implicitly {@link bookStorePackage.BookRequest.verify|verify} messages.
         * @function encodeDelimited
         * @memberof bookStorePackage.BookRequest
         * @static
         * @param {bookStorePackage.IBookRequest} message BookRequest message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BookRequest.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a BookRequest message from the specified reader or buffer.
         * @function decode
         * @memberof bookStorePackage.BookRequest
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {bookStorePackage.BookRequest} BookRequest
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BookRequest.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            let end = length === undefined ? reader.len : reader.pos + length, message = new $root.bookStorePackage.BookRequest();
            while (reader.pos < end) {
                let tag = reader.uint32();
                switch (tag >>> 3) {
                case 1: {
                        message.id = reader.int32();
                        break;
                    }
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a BookRequest message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof bookStorePackage.BookRequest
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {bookStorePackage.BookRequest} BookRequest
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BookRequest.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a BookRequest message.
         * @function verify
         * @memberof bookStorePackage.BookRequest
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        BookRequest.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isInteger(message.id))
                    return "id: integer expected";
            return null;
        };

        /**
         * Creates a BookRequest message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof bookStorePackage.BookRequest
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {bookStorePackage.BookRequest} BookRequest
         */
        BookRequest.fromObject = function fromObject(object) {
            if (object instanceof $root.bookStorePackage.BookRequest)
                return object;
            let message = new $root.bookStorePackage.BookRequest();
            if (object.id != null)
                message.id = object.id | 0;
            return message;
        };

        /**
         * Creates a plain object from a BookRequest message. Also converts values to other types if specified.
         * @function toObject
         * @memberof bookStorePackage.BookRequest
         * @static
         * @param {bookStorePackage.BookRequest} message BookRequest
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        BookRequest.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            let object = {};
            if (options.defaults)
                object.id = 0;
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            return object;
        };

        /**
         * Converts this BookRequest to JSON.
         * @function toJSON
         * @memberof bookStorePackage.BookRequest
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        BookRequest.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        /**
         * Gets the default type url for BookRequest
         * @function getTypeUrl
         * @memberof bookStorePackage.BookRequest
         * @static
         * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
         * @returns {string} The default type url
         */
        BookRequest.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
            if (typeUrlPrefix === undefined) {
                typeUrlPrefix = "type.googleapis.com";
            }
            return typeUrlPrefix + "/bookStorePackage.BookRequest";
        };

        return BookRequest;
    })();

    bookStorePackage.BooksList = (function() {

        /**
         * Properties of a BooksList.
         * @memberof bookStorePackage
         * @interface IBooksList
         * @property {Array.<bookStorePackage.IBookItem>|null} [books] BooksList books
         */

        /**
         * Constructs a new BooksList.
         * @memberof bookStorePackage
         * @classdesc Represents a BooksList.
         * @implements IBooksList
         * @constructor
         * @param {bookStorePackage.IBooksList=} [properties] Properties to set
         */
        function BooksList(properties) {
            this.books = [];
            if (properties)
                for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * BooksList books.
         * @member {Array.<bookStorePackage.IBookItem>} books
         * @memberof bookStorePackage.BooksList
         * @instance
         */
        BooksList.prototype.books = $util.emptyArray;

        /**
         * Creates a new BooksList instance using the specified properties.
         * @function create
         * @memberof bookStorePackage.BooksList
         * @static
         * @param {bookStorePackage.IBooksList=} [properties] Properties to set
         * @returns {bookStorePackage.BooksList} BooksList instance
         */
        BooksList.create = function create(properties) {
            return new BooksList(properties);
        };

        /**
         * Encodes the specified BooksList message. Does not implicitly {@link bookStorePackage.BooksList.verify|verify} messages.
         * @function encode
         * @memberof bookStorePackage.BooksList
         * @static
         * @param {bookStorePackage.IBooksList} message BooksList message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BooksList.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.books != null && message.books.length)
                for (let i = 0; i < message.books.length; ++i)
                    $root.bookStorePackage.BookItem.encode(message.books[i], writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
            return writer;
        };

        /**
         * Encodes the specified BooksList message, length delimited. Does not implicitly {@link bookStorePackage.BooksList.verify|verify} messages.
         * @function encodeDelimited
         * @memberof bookStorePackage.BooksList
         * @static
         * @param {bookStorePackage.IBooksList} message BooksList message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BooksList.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a BooksList message from the specified reader or buffer.
         * @function decode
         * @memberof bookStorePackage.BooksList
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {bookStorePackage.BooksList} BooksList
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BooksList.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            let end = length === undefined ? reader.len : reader.pos + length, message = new $root.bookStorePackage.BooksList();
            while (reader.pos < end) {
                let tag = reader.uint32();
                switch (tag >>> 3) {
                case 1: {
                        if (!(message.books && message.books.length))
                            message.books = [];
                        message.books.push($root.bookStorePackage.BookItem.decode(reader, reader.uint32()));
                        break;
                    }
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a BooksList message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof bookStorePackage.BooksList
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {bookStorePackage.BooksList} BooksList
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BooksList.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a BooksList message.
         * @function verify
         * @memberof bookStorePackage.BooksList
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        BooksList.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.books != null && message.hasOwnProperty("books")) {
                if (!Array.isArray(message.books))
                    return "books: array expected";
                for (let i = 0; i < message.books.length; ++i) {
                    let error = $root.bookStorePackage.BookItem.verify(message.books[i]);
                    if (error)
                        return "books." + error;
                }
            }
            return null;
        };

        /**
         * Creates a BooksList message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof bookStorePackage.BooksList
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {bookStorePackage.BooksList} BooksList
         */
        BooksList.fromObject = function fromObject(object) {
            if (object instanceof $root.bookStorePackage.BooksList)
                return object;
            let message = new $root.bookStorePackage.BooksList();
            if (object.books) {
                if (!Array.isArray(object.books))
                    throw TypeError(".bookStorePackage.BooksList.books: array expected");
                message.books = [];
                for (let i = 0; i < object.books.length; ++i) {
                    if (typeof object.books[i] !== "object")
                        throw TypeError(".bookStorePackage.BooksList.books: object expected");
                    message.books[i] = $root.bookStorePackage.BookItem.fromObject(object.books[i]);
                }
            }
            return message;
        };

        /**
         * Creates a plain object from a BooksList message. Also converts values to other types if specified.
         * @function toObject
         * @memberof bookStorePackage.BooksList
         * @static
         * @param {bookStorePackage.BooksList} message BooksList
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        BooksList.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            let object = {};
            if (options.arrays || options.defaults)
                object.books = [];
            if (message.books && message.books.length) {
                object.books = [];
                for (let j = 0; j < message.books.length; ++j)
                    object.books[j] = $root.bookStorePackage.BookItem.toObject(message.books[j], options);
            }
            return object;
        };

        /**
         * Converts this BooksList to JSON.
         * @function toJSON
         * @memberof bookStorePackage.BooksList
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        BooksList.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        /**
         * Gets the default type url for BooksList
         * @function getTypeUrl
         * @memberof bookStorePackage.BooksList
         * @static
         * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
         * @returns {string} The default type url
         */
        BooksList.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
            if (typeUrlPrefix === undefined) {
                typeUrlPrefix = "type.googleapis.com";
            }
            return typeUrlPrefix + "/bookStorePackage.BooksList";
        };

        return BooksList;
    })();

    bookStorePackage.Empty = (function() {

        /**
         * Properties of an Empty.
         * @memberof bookStorePackage
         * @interface IEmpty
         */

        /**
         * Constructs a new Empty.
         * @memberof bookStorePackage
         * @classdesc Represents an Empty.
         * @implements IEmpty
         * @constructor
         * @param {bookStorePackage.IEmpty=} [properties] Properties to set
         */
        function Empty(properties) {
            if (properties)
                for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * Creates a new Empty instance using the specified properties.
         * @function create
         * @memberof bookStorePackage.Empty
         * @static
         * @param {bookStorePackage.IEmpty=} [properties] Properties to set
         * @returns {bookStorePackage.Empty} Empty instance
         */
        Empty.create = function create(properties) {
            return new Empty(properties);
        };

        /**
         * Encodes the specified Empty message. Does not implicitly {@link bookStorePackage.Empty.verify|verify} messages.
         * @function encode
         * @memberof bookStorePackage.Empty
         * @static
         * @param {bookStorePackage.IEmpty} message Empty message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Empty.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            return writer;
        };

        /**
         * Encodes the specified Empty message, length delimited. Does not implicitly {@link bookStorePackage.Empty.verify|verify} messages.
         * @function encodeDelimited
         * @memberof bookStorePackage.Empty
         * @static
         * @param {bookStorePackage.IEmpty} message Empty message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Empty.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes an Empty message from the specified reader or buffer.
         * @function decode
         * @memberof bookStorePackage.Empty
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {bookStorePackage.Empty} Empty
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Empty.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            let end = length === undefined ? reader.len : reader.pos + length, message = new $root.bookStorePackage.Empty();
            while (reader.pos < end) {
                let tag = reader.uint32();
                switch (tag >>> 3) {
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes an Empty message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof bookStorePackage.Empty
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {bookStorePackage.Empty} Empty
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Empty.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies an Empty message.
         * @function verify
         * @memberof bookStorePackage.Empty
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        Empty.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            return null;
        };

        /**
         * Creates an Empty message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof bookStorePackage.Empty
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {bookStorePackage.Empty} Empty
         */
        Empty.fromObject = function fromObject(object) {
            if (object instanceof $root.bookStorePackage.Empty)
                return object;
            return new $root.bookStorePackage.Empty();
        };

        /**
         * Creates a plain object from an Empty message. Also converts values to other types if specified.
         * @function toObject
         * @memberof bookStorePackage.Empty
         * @static
         * @param {bookStorePackage.Empty} message Empty
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        Empty.toObject = function toObject() {
            return {};
        };

        /**
         * Converts this Empty to JSON.
         * @function toJSON
         * @memberof bookStorePackage.Empty
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        Empty.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        /**
         * Gets the default type url for Empty
         * @function getTypeUrl
         * @memberof bookStorePackage.Empty
         * @static
         * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
         * @returns {string} The default type url
         */
        Empty.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
            if (typeUrlPrefix === undefined) {
                typeUrlPrefix = "type.googleapis.com";
            }
            return typeUrlPrefix + "/bookStorePackage.Empty";
        };

        return Empty;
    })();

    return bookStorePackage;
})();

export { $root as default };
