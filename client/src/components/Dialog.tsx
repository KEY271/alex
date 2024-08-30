import { useCallback, useEffect, useRef } from "react";

type DialogProps = {
    text: string;
    isOpen: boolean;
    onClose: () => void;
    action: (res: boolean) => () => void;
};

function Dialog(props: DialogProps) {
    const { text, isOpen, onClose, action } = props;

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
    },[]);

    return (
        <dialog ref={ref} onClose={onClose}>
            <div onClick={handleClickContent} className="p-4 flex flex-col gap-4">
                <p>{text}</p>
                <div className="flex justify-between">
                    <button className="p-2 hover:bg-[lightsalmon] transition rounded" onClick={action(true)}>はい</button>
                    <button className="p-2 hover:bg-[lightsalmon] transition rounded" onClick={action(false)}>いいえ</button>
                </div>
            </div>
        </dialog>
    )
}

export default Dialog;
