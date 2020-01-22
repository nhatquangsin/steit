using System;
using System.Collections.Generic;

using Steit;
using Steit.Collections;
using Steit.Encoding;
using Steit.State;

namespace Steit.Test1 {
    public sealed class Hello : IState {
        private static IList<Listener<StateList<Int32>>> numbersListeners = new List<Listener<StateList<Int32>>>();

        public Path Path { get; private set; }

        public StateList<Int32> Numbers { get; private set; }

        public Hello(Path path = null) {
            this.Path = path != null ? path : Path.Root;
            this.Numbers = new StateList<Int32>(this.Path.Nested(0));
        }

        public delegate void Listener<T>(T newValue, T oldValue, Hello container);

        public static int OnUpdateNumbers(Listener<StateList<Int32>> listener) { return Utilities.Add(numbersListeners, listener); }

        public static void RemoveNumbersListener(Listener<StateList<Int32>> listener) { numbersListeners.Remove(listener); }

        public static void RemoveNumbersListenerAt(int index) { numbersListeners.RemoveAt(index); }

        public static void ClearNumbersListeners() { numbersListeners.Clear(); }

        public static void ClearAllListeners() {
            numbersListeners.Clear();
        }

        public static Hello Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {
            var hello = new Hello(path);
            hello.ReplaceAll(reader, shouldNotify);
            return hello;
        }

        public Int16 WireType(UInt16 tag) {
            switch (tag) {
                case 0: return (Int16) Steit.Encoding.WireType.Sized;
                default: return -1;
            }
        }

        public IState Nested(UInt16 tag) {
            switch (tag) {
                case 0: return this.Numbers;
                default: return null;
            }
        }

        public bool IsAddSupported() { return false; }
        public bool IsRemoveSupported() { return false; }

        public void ReplayAdd(Reader reader) { throw new Exception("Not supported"); }
        public void ReplayRemove(UInt16 tag) { throw new Exception("Not supported"); }

        public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify) {
            switch (tag) {
                case 0: this.Numbers = this.Notify(StateList<Int32>.Deserialize(reader.Nested(), this.Path.Nested(0)), this.Numbers, shouldNotify, numbersListeners); break;
                default: reader.SkipWireTyped(wireType); break;
            }
        }

        private T Notify<T>(T newValue, T oldValue, bool shouldNotify, IList<Listener<T>> listeners) {
            if (shouldNotify) {
                foreach (var listener in listeners) {
                    listener(newValue, oldValue, this);
                }
            }

            return newValue;
        }
    }
}
