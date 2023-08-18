package dev.redio.chorus.utility.container;

import dev.redio.chorus.utility.func.FuncIntR;

public interface Container<T> extends Iterable<T> {
    
    int length();

    int capacity();

    default T[] toArray(FuncIntR<T[]> func) {
        final var result = func.func(length());
        final var iter = iterator();
        for (int i = 0; i < result.length; i++) 
            result[i] = iter.next();
        
        return result;
    }
}
