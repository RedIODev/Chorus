package dev.redio.chorus.utility.container;

import java.util.Iterator;

public class DynArray<T> implements Container<T> {

    public static final float DEFAULT_GROW_FACTOR = 1.5f;
    private static final Object[] EMPTY_ARRAY = {};
    private int length;
    private final float growFactor;
    private Object[] array;

    public DynArray() {
        this(EMPTY_ARRAY, DEFAULT_GROW_FACTOR);
    }

    public DynArray(float growFactor) {
        this(EMPTY_ARRAY, growFactor);
    }

    private DynArray(Object[] array, float growFactor) {
        this.growFactor = growFactor;
        this.array = array;
    }

    @Override
    public Iterator<T> iterator() {
        return new Iterator<T>() {

            private int index = -1;

            @Override
            public boolean hasNext() {
                return index < length;
            }

            @SuppressWarnings("unchecked")
            @Override
            public T next() {
                index++;
                return (T) array[index];
            }
            
        };
    }

    @SuppressWarnings("unchecked")
    public Option<T> get(int index) {
        if (index < 0 || index >= length)
            return Option.none();
        return Option.some((T)array[index]);
    }

    public boolean set(int index, T value) {
        if (index < 0 || index >= length)
            return false;
        array[index] = value;
        return true;
    }

    public Option<T> remove(int index) {
        final var result = get(index);
        if (result instanceof Option.None)
            return result;
        if (index == length-1) {
            array[index] = null;
            length--;
            return result;
        }
        System.arraycopy(array, index + 1, array, index, length - index);
        length--;
        return result;
    }

    public boolean add(T value) {
        return add(length, value);
    }

    public boolean add(int index, T value) {
        if (index < 0 || index >= length)
            return false;
        if (length>= array.length)
            grow();
            
        if (index != length) 
            System.arraycopy(array, index, array, index + 1, length - index);
                
        array[index] = value;
        return true;

    }

    @Override
    public int length() {
        return this.length;
    }

    @Override
    public int capacity() {
        return array.length;
    }

    public void reserve(int elements) {
        this.grow(elements + length);
    }

    public void trim() {
        var newArray = new Object[length];
        System.arraycopy(array, 0, newArray, 0, length);
        array = newArray;
    }

    private void grow() {
        this.grow((int)(this.length * growFactor));
    }
    
    private void grow(int newLength) {
        if (newLength == 0)
            return;
        var newArray = new Object[newLength];
        System.arraycopy(this.array, 0, newArray, 0, this.array.length);
        this.array = newArray;
    } 



    
    
}
