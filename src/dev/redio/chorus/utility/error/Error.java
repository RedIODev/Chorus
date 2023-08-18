package dev.redio.chorus.utility.error;

import dev.redio.chorus.utility.container.Option;

public interface Error {
    
    default Throwable throwable() {
        return new Throwable(this.toString());
    }

    default Option<Error> previous() {
        return Option.none();
    }

    static Error fromThrowable(Throwable t) {
        return new Error() {
            
            @Override
            public Throwable throwable() {
                return t;
            }

            @Override
            public Option<Error> previous() {
                final var cause = t.getCause();
                if (cause == null)
                    return Option.none();
                return Option.some(fromThrowable(cause));
            }
        };
    }
}
