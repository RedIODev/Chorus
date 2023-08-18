package dev.redio.chorus.utility.container;

import dev.redio.chorus.utility.error.Error;
import dev.redio.chorus.utility.func.FuncR;
import dev.redio.chorus.utility.func.FuncTR;

public sealed interface Option<T> {
    public record Some<T>(T value) implements Option<T> {
        public Some {
            if (value == null)
                throw new IllegalArgumentException("Option.Some.value can't be null");
        }
    }

    public record None<T>() implements Option<T> {
        private static final None<?> EMPTY = new None<>();
    }
    
    default T someOr(T other) {
        if (this instanceof Some<T> s)
            return s.value;
        return other;
    }

    default T someOrGen(FuncR<T> func) {
        if (this instanceof Some<T> s)
            return s.value;
        return func.func();
    }

    default <U> Option<U> map(FuncTR<T,U> func) {
        if (this instanceof Some<T> s)
            return some(func.func(s.value));
        return none();
    }

    default <E extends Error> Result<T, E> asResult(E error) {
        if (this instanceof Some<T> s)
            return Result.ok(s.value);
        return Result.error(error);
    }

    default <E extends Error> Result<T,E> asResultGen(FuncR<E> func) {
        if (this instanceof Some<T> s) 
            return Result.ok(s.value);
        return Result.error(func.func());
    }

    static <T> Option<T> some(T value) {
        if (value == null)
            return new None<>();
        return new Some<>(value);
    }

    @SuppressWarnings("unchecked")
    static <T> None<T> none() {
        return (None<T>)None.EMPTY;
    }

    static <T> Option<T> flatten(Option<Option<T>> option) {
        if (option instanceof Some<Option<T>> some)
            return some.value;
        return none();
    }

}
