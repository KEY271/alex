import { PieceType } from "../utils/game";

function piecename(pt: PieceType): string {
    switch (pt) {
        case PieceType.None:
            return "";
        case PieceType.Light:
            return "歩";
        case PieceType.Heavy:
            return "重";
        case PieceType.King:
            return "玉";
        case PieceType.Prince:
            return "子";
        case PieceType.General:
            return "将";
        case PieceType.Knight:
            return "騎";
        case PieceType.Arrow:
            return "矢";
        case PieceType.Archer0:
            return "弓";
        case PieceType.Archer1:
            return "弓1";
        case PieceType.Archer2:
            return "弓2";
    }
}

type PieceProps = {
    pt: PieceType;
};

function Piece(props: PieceProps) {
    const { pt } = props;
    const name = piecename(pt);
    return (
        <div
            className="flex h-8 w-8 items-center justify-center bg-black
                [clip-path:polygon(50%_0,90%_15%,100%_100%,0_100%,10%_15%)] sm:h-12 sm:w-12">
            <div
                className="flex h-7 w-7 justify-center bg-[burlywood] pt-1 text-base
                    [clip-path:polygon(50%_0,90%_15%,100%_100%,0_100%,10%_15%)] sm:h-11 sm:w-11 sm:pt-2 sm:text-2xl">
                {name}
            </div>
        </div>
    );
}

export default Piece;
