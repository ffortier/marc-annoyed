import init, { MarcAnnoyed } from 'marc-annoyed';

init().then(() => {
    const game = new MarcAnnoyed({
        parent: document.body,
    });

    game.run();
});