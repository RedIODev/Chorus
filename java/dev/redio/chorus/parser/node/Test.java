package dev.redio.chorus.parser.node;

import dev.redio.chorus.processor.Frozen;
import dev.redio.chorus.processor.Mutable;

@Mutable
public record Test<T,U extends Iterable<String> & CharSequence>(T t, U u, @Frozen Test2 t2) {
    
}
