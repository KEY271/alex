import { useCallback, useEffect, useRef } from "react";

type DialogProps<T> = {
    text: string;
    isOpen: boolean;
    onClose: () => void;
    candidates: [string, T][];
    action: (res: T) => () => void;
};

function Dialog<T>(props: DialogProps<T>) {
    const { text, isOpen, onClose, candidates, action } = props;

    const ref = useRef<HTMLDialogElement>(null);
    useEffect(() => {
        const elem = ref.current;
        if (!elem) {
            return;
        }
        if (isOpen) {
            if (elem.hasAttribute("open")) {
                return;
            }
            elem.showModal();
        } else {
            if (!elem.hasAttribute("open")) {
                return;
            }
            elem.close();
        }
    }, [isOpen]);

    const handleClickContent = useCallback((event: React.MouseEvent<HTMLDivElement>): void => {
        event.stopPropagation();
    }, []);

    const buttons = candidates.map(([t, value], i) => {
        return (
            <button key={i} className="rounded p-2 transition hover:bg-[lightsalmon]" onClick={action(value)}>
                {t}
            </button>
        );
    });

    return (
        <dialog ref={ref} onClose={onClose}>
            <div onClick={handleClickContent} className="flex flex-col gap-4 p-4">
                <p>{text}</p>
                <div className="flex justify-between">{buttons}</div>
            </div>
        </dialog>
    );
}

export default Dialog;
