import 'package:appflowy/generated/locale_keys.g.dart';
import 'package:appflowy_editor/appflowy_editor.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

import '../util/util.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('create and delete the document', () {
    testWidgets('create a new document when launching app in first time',
        (tester) async {
      await tester.initializeAppFlowy();

      await tester.tapGoButton();

      // create a new document
      await tester.createNewPageWithName();

      // expect to see a new document
      tester.expectToSeePageName(
        LocaleKeys.menuAppHeader_defaultNewPageName.tr(),
      );
      // and with one paragraph block
      expect(find.byType(TextBlockComponentWidget), findsOneWidget);
    });

    testWidgets('delete the readme page and restore it', (tester) async {
      await tester.initializeAppFlowy();

      await tester.tapGoButton();

      // delete the readme page
      await tester.hoverOnPageName(
        gettingStated,
        onHover: () async => await tester.tapDeletePageButton(),
      );

      // the banner should show up and the readme page should be gone
      tester.expectToSeeDocumentBanner();
      tester.expectNotToSeePageName(gettingStated);

      // restore the readme page
      await tester.tapRestoreButton();

      // the banner should be gone and the readme page should be back
      tester.expectNotToSeeDocumentBanner();
      tester.expectToSeePageName(gettingStated);
    });

    testWidgets('delete the readme page and delete it permanently',
        (tester) async {
      await tester.initializeAppFlowy();

      await tester.tapGoButton();

      // delete the readme page
      await tester.hoverOnPageName(
        gettingStated,
        onHover: () async => await tester.tapDeletePageButton(),
      );

      // the banner should show up and the readme page should be gone
      tester.expectToSeeDocumentBanner();
      tester.expectNotToSeePageName(gettingStated);

      // delete the page permanently
      await tester.tapDeletePermanentlyButton();

      // the banner should be gone and the readme page should be gone
      tester.expectNotToSeeDocumentBanner();
      tester.expectNotToSeePageName(gettingStated);
    });
  });
}
