import type { UserLookup } from "./user";
import { compareUsername, missingUserIds, userIsOnline } from "./user.utils";

const now = Date.now();
jest.setSystemTime(now);

const lookup: UserLookup = {
    a: {
        userId: "a",
        username: "a",
        lastOnline: now - 119 * 1000,
    },
    b: {
        userId: "b",
        username: "b",
        lastOnline: now - 200 * 1000,
    },
};

describe("get user status", () => {
    test("user is online", () => {
        expect(userIsOnline(lookup, "a")).toBe(true);
    });

    test("user is offline", () => {
        expect(userIsOnline(lookup, "b")).toBe(false);
    });

    test("unknown user is considered offline", () => {
        expect(userIsOnline(lookup, "c")).toBe(false);
    });
});

describe("missing userIds", () => {
    test("should work", () => {
        const missing = missingUserIds(lookup, new Set(["a", "b", "c", "d", "e"]));
        ["c", "d", "e"].forEach((u) => expect(missing.includes(u)).toBe(true));
    });
});

describe("compare username", () => {
    function toUser(username: string | undefined) {
        return { userId: "a", username, lastOnline: now };
    }
    test("works with non-null usernames", () => {
        const users = ["zulu", "yanky", "foxtrot", "lima"].map(toUser);
        const sorted = users.sort(compareUsername);
        expect(sorted.map((u) => u.username)).toEqual(["foxtrot", "lima", "yanky", "zulu"]);
    });

    test("works with non-null usernames", () => {
        const users = ["zulu", undefined, "yanky", undefined, "foxtrot", "lima"].map(toUser);
        const sorted = users.sort(compareUsername);
        expect(sorted.map((u) => u.username)).toEqual([
            "foxtrot",
            "lima",
            "yanky",
            "zulu",
            undefined,
            undefined,
        ]);
    });
});
