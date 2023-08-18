package dev.redio.chorus.utility.container;

import java.util.Iterator;

import dev.redio.chorus.utility.func.FuncIntR;
import dev.redio.chorus.utility.func.FuncIntTboolean;

public final class View<T> implements Container<T> {

    private final int start;
    private final int length;
    private final FuncIntR<Option<T>> getter;
    private final FuncIntTboolean<T> setter;

    private View(int start, int length, FuncIntR<Option<T>> getter, FuncIntTboolean<T> setter) {
        this.start = start;
        this.length = length;
        this.getter = getter;
        this.setter = setter;
    }

    public Option<T> get(int index) {
        if (index < 0 || index >= length)
            return Option.none();
        return getter.func(index + start);
    }

    public boolean set(int index, T value) {
        if (index < 0 || index >= length)
            return false;
        return setter.func(start + index, value);
    }

    @Override
    public Iterator<T> iterator() {
        return new Iterator<>() {
            int index = -1;
            @Override
            public boolean hasNext() {
                return index < length;
            }

            @Override
            public T next() {
                index++;
                return get(index).someOr(null);
            }
            
        };
    }

    @Override
    public int length() {
        return length;
    }

    @Override
    public int capacity() {
        return length;    
    }
    
    public Option<View<T>> subView(int start, int length) {
        final var newStart = start + this.start;
        if (newStart >= this.length || newStart < this.start)
            return Option.none();
        if (newStart + length > this.start + this.length)
            return Option.none();
        return Option.some(new View<>(start+ this.start, length, getter, setter));
    }
    
}
