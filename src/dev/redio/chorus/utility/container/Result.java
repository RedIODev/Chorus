package dev.redio.chorus.utility.container;

import dev.redio.chorus.utility.func.FuncR;
import dev.redio.chorus.utility.func.FuncTR;

public sealed interface Result<T,E extends dev.redio.chorus.utility.error.Error> {
    public record Ok<T,E extends dev.redio.chorus.utility.error.Error>(T value) implements Result<T,E> {
        public Ok {
            if (value == null)
                throw new IllegalArgumentException("Result.Some.value can't be null");
        }
    }

    public record Error<T,E extends dev.redio.chorus.utility.error.Error>(E error) implements Result<T,E> {
        public Error {
            if (error == null)
                throw new IllegalArgumentException("Result.Error.error can't be null");
        }
    }

    default T someOr(T value) {
        if (this instanceof Ok<T,E> o)
            return o.value;
        return value;
    }

    default T someOrGen(FuncR<T> func) {
        if (this instanceof Ok<T,E> o)
            return o.value;
        return func.func();
    }

    default E errorOr(E error) {
        if (this instanceof Error<T,E> e)
            return e.error;
        return error;
    }

    default E errorOrGen(FuncR<E> func) {
        if (this instanceof Error<T,E> e) 
            return e.error;
        return func.func();
    }

    default <U,F extends dev.redio.chorus.utility.error.Error> Result<U,F> map(FuncTR<T,U> funcOk, FuncTR<E,F> funcError) {
        return switch (this) {
            case Ok<T,E> o -> ok(funcOk.func(o.value));
            case Error<T,E> e -> error(funcError.func(e.error));
        };
    }

    default <U> Result<U,E> mapSome(FuncTR<T,U> func) {
        return switch (this) {
            case Ok<T,E> o -> ok(func.func(o.value));
            case Error<T,E> e -> error(e.error);
        };
    }

    default <F extends dev.redio.chorus.utility.error.Error> Result<T,F> mapError(FuncTR<E,F> func) {
        return switch (this) {
            case Ok<T,E> o -> ok(o.value);
            case Error<T,E> e -> error(func.func(e.error));
        };
    }

    default Option<T> asOptionOk() {
        if (this instanceof Ok<T,E> o)
            return Option.some(o.value);
        return Option.none();
    }

    default Option<E> asOptionError() {
        if (this instanceof Error<T,E> e) 
            return Option.some(e.error);
        return Option.none();
    }

    static <T, E extends dev.redio.chorus.utility.error.Error> Result<T,E> ok(T value) {
        return new Ok<>(value);
    } 

    static <T, E extends dev.redio.chorus.utility.error.Error> Result<T,E> error(E error) {
        return new Error<T,E>(error);
    }

    static <T, E extends dev.redio.chorus.utility.error.Error, F extends E, G extends E> Result<T,E> flatten(Result<Result<T,F>,G> result) {
        return switch (result) {
            case Ok<Result<T,F>,G>(Ok<T,F> o) -> ok(o.value); 
            case Ok<Result<T,F>,G>(Error<T,G> e) -> error(e.error);
            case Error<Result<T,F>,G> e -> error(e.error);
        };
    }
}
